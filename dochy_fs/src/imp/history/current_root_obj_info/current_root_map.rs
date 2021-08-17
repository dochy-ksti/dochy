use once_cell::sync::Lazy;
use parking_lot::{Mutex, MutexGuard};
use std::collections::HashMap;
use crate::common::{DochyCache, CurrentSrc};
use crate::history::{FileNameProps, HistoryOptions};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::path::{PathBuf, Path};
use crate::error::FsResult;
use crate::imp::history::history_info::HistoryInfo;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Weak};
use dochy_core::structs::RootObject;

#[derive(Debug, Clone)]
pub struct CurrentRootObjInfo {
    current_root_id: Weak<()>,
    current_base_file: FileNameProps,
}

impl CurrentRootObjInfo {
    pub fn new(current_root_id: Weak<()>, current_base_file: FileNameProps) -> CurrentRootObjInfo {
        CurrentRootObjInfo { current_root_id, current_base_file}
    }

    pub fn current_root_id(&self) -> &Weak<()>{ &self.current_root_id }
    pub fn current_base_file(&self) -> &FileNameProps{ &self.current_base_file }
}

struct MapItem{
    queued : AtomicUsize,
    current_src : CurrentSrc,
    hash : u128,
    history_options : HistoryOptions,
    src_root : RootObject,
    synced : Box<Mutex<SyncedItem>>
}

pub(crate) struct SyncedItem{
    cache : DochyCache,
    current_root : Option<CurrentRootObjInfo>,
}

impl SyncedItem{
    pub(crate) fn muts(&mut self) -> (&mut DochyCache, &mut Option<CurrentRootObjInfo>){
        (&mut self.cache, &mut self.current_root)
    }
}

pub(crate) struct MutexG<'a>{
    guard : MutexGuard<'a, SyncedItem>,
    history_dir : PathBuf,
    current_info : CurrentRootInfo,
}
impl<'a> MutexG<'a>{
    //pub fn history_dir(&self) -> &Path{ &self.history_dir }
    pub(crate) fn current_info(&self) -> &CurrentRootInfo{ &self.current_info }
    pub(crate) fn set_current_root_obj_info(&mut self, current_root_obj_info : Option<CurrentRootObjInfo>) {
        *self.guard.current_root = current_root_obj_info;
    }
}
impl<'a> Drop for MutexG<'a>{
    fn drop(&mut self) {
        dec_num_queue(&self.history_dir).ok();
    }
}

impl<'a> Deref for MutexG<'a>{
    type Target = SyncedItem;

    fn deref(&self) -> &Self::Target {
        Deref::deref(&self.guard)
    }
}

impl<'a> DerefMut for MutexG<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        DerefMut::deref_mut(&mut self.guard)
    }
}

static MAP : Lazy<Mutex<HashMap<PathBuf, Box<MapItem>>>> = Lazy::new(||{
    Mutex::new(HashMap::new())
});

pub(crate) fn init_dochy_cache(history_dir : &Path, current_src : CurrentSrc, op : &HistoryOptions) -> FsResult<HistoryInfo>{
    let mut map = MAP.lock();
    if let Some(item) = map.get(history_dir){
        if item.current_src != current_src{
            Err("Source alternation while running is not supported")?
        } else{
            return Ok(HistoryInfo::new(history_dir.to_path_buf()));
        }
    }

    let cache = DochyCache::new(current_src.clone())?;
    let hash = cache.hash();
    //let src_root = cache.clone_src_root();
    map.insert(history_dir.to_path_buf(), Box::new(MapItem{
        hash,
        queued : AtomicUsize::new(0),
        current_src,
        history_options : op.clone(),
        //src_root,
        synced : Box::new(Mutex::new(SyncedItem{
            cache,
            current_root : None
        }))
    }));
    return Ok(HistoryInfo::new(history_dir.to_path_buf()));
}

///unbound life time だが、Boxのアドレスは固定なので安全なはず
pub(crate) fn get_map_item<'a>(history_dir : &Path) -> FsResult<&'a MapItem>{
    let ptr: *const MapItem = {
        let mut map = MAP.lock();
        let b = map.get(history_dir)?;
        b.as_ref()
    };
    Ok(unsafe{ &*ptr })
}

fn dec_num_queue(history_dir : &Path) -> FsResult<()>{
    let m = get_map_item(history_dir)?;
    m.queued.fetch_sub(1, Ordering::Relaxed);
    Ok(())
}

pub(crate) fn get_mutex<'a>(history_dir : &Path) -> FsResult<MutexG<'a>>{
    let item = get_map_item(history_dir)?;
    item.queued.fetch_add(1, Ordering::Relaxed);
    let guard = item.synced.lock();
    let current_info = get_current_root_info_impl(item, &guard);
    Ok(MutexG{ guard, history_dir : history_dir.to_path_buf(), current_info })
}

#[derive(Debug, Clone)]
pub struct CurrentRootInfo{
    hash : u128,
    queued : usize,
    current_src : CurrentSrc,
    history_options : HistoryOptions,
    current_root_obj_info : Option<CurrentRootObjInfo>,
}

impl CurrentRootInfo{
    pub fn hash(&self) -> u128{ self.hash }
    pub fn queued(&self) -> usize{ self.queued }
    pub fn current_src(&self) -> &CurrentSrc{ &self.current_src }
    pub fn history_options(&self) -> &HistoryOptions{ &self.history_options }
    ///あとで消せるなら消す
    pub fn current_root_obj_info(&self) -> &Option<CurrentRootObjInfo>{ &self.current_root_obj_info }
}

/// how many threads waiting to save_async
pub fn peek_num_queued_threads(history_info : &HistoryInfo) -> FsResult<usize> {
    let item = get_map_item(history_info.history_dir())?;
    Ok(item.queued.into())
}

/// You can peek the file to be derived in the next save, but the Mutex is needed for save and load.
/// If you call save or load while the MutexGuard is alive, deadlock occurs.
pub fn get_current_root_info(history_info : &HistoryInfo) -> FsResult<CurrentRootInfo>{
    let item = get_map_item(history_info.history_dir())?;
    let m = item.synced.lock();
    Ok(get_current_root_info_impl(item, &m))
}

fn get_current_root_info_impl(item : &MapItem, m : &MutexGuard<SyncedItem>) -> CurrentRootInfo{
    let hash = m.cache.hash();
    let queued : usize = item.queued.into();
    let current_root_obj_info = m.current_root.clone();
    let current_src = item.current_src.clone();
    let history_options = item.history_options.clone();

    CurrentRootInfo{
        hash,
        queued,
        current_src,
        current_root_obj_info,
        history_options,
    }
}

/// During save and load, the RootObject's ID and the selected file is recorded. If you use the same RootObject in the next save,
/// the file to be derived is automatically selected by the system.
///
/// This is the backdoor. You can set the ID and a file info and designate the file to be derived in the next save.
/// Arbitrary deriving is not supported. You must derive from an older state of the RootObject.
///
/// If you call this function before the MutexGuard is dropped, deadlock occurs.
pub fn set_current_root_obj_info(history_info : &HistoryInfo, current_root_obj_info : Option<CurrentRootObjInfo>) -> FsResult<()>{
    let m = get_map_item(history_info.history_dir())?;
    let mut s = m.synced.lock();
    s.current_root = current_root_obj_info;
    Ok(())
}