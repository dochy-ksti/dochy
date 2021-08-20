use crate::common::CurrentSrc;
use std::path::{PathBuf, Path};
use dochy_core::structs::RootObject;
use crate::imp::filesys::save_cache_map::get_save_cache;
use crate::error::FsResult;
use crate::imp::filesys::save_dir_info::SaveDirInfo;

#[derive(Debug, Clone)]
pub struct SaveCacheItem{
    info : SaveDirInfo
}

impl SaveCacheItem{
    pub(crate) fn new(info : SaveDirInfo) -> SaveCacheItem{
        SaveCacheItem{ info }
    }

    pub fn save_dir(&self) -> &Path{ self.info.save_dir() }
    pub fn current_src(&self) -> &CurrentSrc{ self.info.current_src() }
    pub fn hash(&self) -> u128{ self.info.hash() }
    pub fn clone_src_root(&self) -> RootObject{ self.info.clone_src_root() }
}