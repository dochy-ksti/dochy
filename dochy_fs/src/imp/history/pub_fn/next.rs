use crate::error::FsResult;
use std::path::Path;
use crate::history::HistoryOptions;
use dochy_core::structs::RootObject;
use crate::imp::history::fs::next::next as fs_next;
use crate::imp::history::file_hist::prepare_history_hash_dir::prepare_history_hash_dir;
use crate::imp::history::diff_and_cache::dochy_cache::DochyCache;

/// calculates diff from the last state and save the diff as a history file
pub fn next<P : AsRef<Path>>(history_dir: P,
                             tag : Option<String>,
                             root : &RootObject,
                             cache : &mut DochyCache,
                             options : &HistoryOptions) -> FsResult<()> {
    let history_dir = history_dir.as_ref();
    let src = cache.current_src();

    let history_hash_dir = prepare_history_hash_dir(history_dir, src)?;
    fs_next(tag, root, cache, history_hash_dir, options)
}
