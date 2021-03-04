use crate::fs::error::FsResult;
use std::path::Path;
use crate::fs::history::HistoryOptions;
use crate::core::structs::RootObject;
use crate::fs::imp::history::fs::next::next as fs_next;
use crate::fs::imp::history::file_hist::prepare_history_hash_dir::prepare_history_hash_dir;
use crate::fs::imp::history::diff_and_cache::dochy_cache::DochyCache;

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
