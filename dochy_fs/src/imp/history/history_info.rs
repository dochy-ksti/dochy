use std::path::{PathBuf, Path};
use crate::common::CurrentSrc;
use crate::imp::history::current_root_obj_info::history_cache_map::{init_dochy_cache};
use crate::error::FsResult;
use crate::history::{HistoryOptions, get_peekable_info, PeekableCacheInfo};
use dochy_core::structs::RootObject;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct HistoryInfo{
    history_dir : PathBuf,
}

impl HistoryInfo{
    pub(crate) fn new(history_dir : PathBuf) -> HistoryInfo{
        HistoryInfo{ history_dir }
    }
    pub fn create<
        P : AsRef<Path>,
        Op : AsRef<HistoryOptions>>(history_dir : P,
                                    current_src : CurrentSrc,
                                    history_options : Op) -> FsResult<HistoryInfo>{
        init_dochy_cache(history_dir.as_ref(), current_src, history_options.as_ref())
    }
    pub fn history_dir(&self) -> &Path{ &self.history_dir }
    pub fn peekable(&self) -> &PeekableCacheInfo{
        get_peekable_info(self).unwrap()
    }
    pub fn clone_src_root(&self) -> RootObject{
        self.peekable().clone_src_root()
    }
}