use once_cell::sync::Lazy;
use parking_lot::{Mutex, MutexGuard};
use std::collections::HashMap;
use crate::common::{DochyCache, CurrentSrc, JSON_ARC_OPT};
use crate::history::{HistoryOptions, PeekableCacheInfo};
use std::path::{PathBuf, Path};
use crate::error::FsResult;
use crate::imp::history::history_info::HistoryInfo;
use crate::imp::history::current_root_obj_info::current_root_obj_info::CurrentRootObjInfo;
use crate::imp::history::current_root_obj_info::history_cache_item::{SyncedItem, HistoryCacheItem};
use crate::imp::history::current_root_obj_info::mutex_g::MutexG;
use crate::imp::common::current_src_info::CurrentSrcInfo;
use dochy_core::structs::RootObject;
use dochy_core::json_dir_to_root;
use dochy_archiver::get_hash_and_metadata_from_dir;
use crate::imp::common::archive::load_archive::load_archive_and_hash;


static MAP : Lazy<Mutex<HashMap<CurrentSrc, Box<CurrentSrcInfo>>>> = Lazy::new(||{
    Mutex::new(HashMap::new())
});

pub(crate) fn get_current_src_cache(current_src : CurrentSrc) -> FsResult<CurrentSrcInfo>{
    let mut map = MAP.lock();
    if let Some(item) = map.get(&current_src){
        return Ok(item.as_ref().clone())
    }
    let (root, hash) = create_root_and_hash(&current_src)?;
    let info = CurrentSrcInfo::new(current_src.clone(), root, hash);
    map.insert(current_src, Box::new(info.clone()));
    Ok(info)
}

fn create_root_and_hash(current_src : &CurrentSrc) -> FsResult<(RootObject, u128)>{
    match current_src{
        CurrentSrc::SrcDir(src_dir) => {
            let root = json_dir_to_root(src_dir, validation)?;
            let (hash, _meta) = get_hash_and_metadata_from_dir(src_dir, &JSON_ARC_OPT)?;
            Ok((root, hash))
        },
        CurrentSrc::ArchiveFile(path) =>{
            load_archive_and_hash(path, validation)
        }
    }
}
