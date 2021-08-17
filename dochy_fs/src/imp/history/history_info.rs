use std::path::{PathBuf, Path};
use crate::common::CurrentSrc;
use crate::imp::history::current_root_obj_info::current_root_map::{init_dochy_cache, get_mutex};
use crate::error::FsResult;
use crate::history::HistoryOptions;
use dochy_core::structs::RootObject;

#[derive(Debug, Eq, PartialEq, Hash)]
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
    pub fn clone_src_root(&self) -> RootObject{

    }
}