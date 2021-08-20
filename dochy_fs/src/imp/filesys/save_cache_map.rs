use once_cell::sync::Lazy;
use parking_lot::{Mutex, MutexGuard};
use std::collections::HashMap;
use crate::common::{DochyCache, CurrentSrc};
use crate::history::{HistoryOptions, PeekableCacheInfo};
use std::path::{PathBuf, Path};
use crate::error::FsResult;
use crate::imp::history::history_info::HistoryInfo;
use crate::imp::history::current_root_obj_info::current_root_obj_info::CurrentRootObjInfo;
use crate::imp::history::current_root_obj_info::history_cache_item::{SyncedItem, HistoryCacheItem};
use crate::imp::history::current_root_obj_info::mutex_g::MutexG;
use crate::imp::filesys::save_dir_info::SaveDirInfo;
use crate::imp::common::current_src_map::get_current_src_cache;
use crate::imp::common::prepare_hash_dir::prepare_hash_dir;
use crate::imp::filesys::save_cache_item::SaveCacheItem;


static MAP : Lazy<Mutex<HashMap<PathBuf, Box<Mutex<SaveCacheItem>>>>> = Lazy::new(||{
    Mutex::new(HashMap::new())
});

pub(crate) fn cache_and_get_info(save_dir : &Path,
                                          current_src : CurrentSrc) -> FsResult<SaveDirInfo>{
    let mut map = MAP.lock();
    if let Some(item) = map.get(save_dir){
        return Ok(item.as_ref().lock()?);
    }
    let info = create_save_dir_info(save_dir, current_src)?;
    map.insert(save_dir.to_path_buf(), Box::new(Mutex::new(SaveCacheItem::new(info.clone()))));
    Ok(info)
}

/// Updates current_src cache corresponding to save_dir.
/// Technically, this is not unsafe, but all the copies of old info are going to be incorrect.
pub unsafe fn cache_and_get_info_us<P : AsRef<Path>>(save_dir : P,
                                                                 current_src : CurrentSrc) -> FsResult<SaveDirInfo>{
    let mut map = MAP.lock();
    let info = create_save_dir_info(save_dir, current_src)?;
    map.insert(save_dir.to_path_buf(),
               Box::new(Mutex::new(SaveCacheItem::new(info.clone()))));
    Ok(info)
}

fn create_save_dir_info(save_dir : &Path, current_src : CurrentSrc) -> FsResult<SaveDirInfo>{
    let current = get_current_src_cache(current_src)?;
    let info = SaveDirInfo::new(save_dir.to_path_buf(),
                                current.current_src().clone(),
                                current.hash(),
                                current.clone_src_root());
    Ok(info)
}

pub(crate) fn get_cache(save_dir : &Path) -> FsResult<MutexGuard<SaveCacheItem>>{
    let mut map = MAP.lock();
    let ptr : *const Mutex<SaveCacheItem> = map.get(save_dir)?.as_ref();
    let mutex = unsafe{ &*ptr };
    Ok(mutex.lock())
}