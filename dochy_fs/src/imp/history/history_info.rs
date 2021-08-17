use std::path::{PathBuf, Path};
use crate::common::CurrentSrc;
use crate::imp::history::current_root_obj_info::current_root_map::init_dochy_cache;
use crate::error::FsResult;

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct HistoryInfo{
    history_dir : PathBuf,
}

impl HistoryInfo{
    pub(crate) fn new(history_dir : PathBuf) -> HistoryInfo{
        HistoryInfo{ history_dir }
    }
    pub fn create<P : AsRef<Path>>(history_dir : P, current_src : CurrentSrc) -> FsResult<HistoryInfo>{
        init_dochy_cache(history_dir.as_ref(), current_src)
    }
    pub fn history_dir(&self) -> &Path{ &self.history_dir }
}