use crate::error::FsResult;
use std::path::Path;
use crate::history::HistoryOptions;
use dochy_core::structs::RootObject;
use crate::imp::history::fs::next::next as fs_next;
use crate::imp::history::file_hist::prepare_history_hash_dir::prepare_history_hash_dir;
use crate::imp::history::diff_and_cache::dochy_cache::DochyCache;

/// calculates the diff from the latest save file(most of the time) and save the diff file.
/// If the 'root' is not derived from the latest save file, calculate diff from the source JSON5 and save it.
/// (RootObject has an ID and when the latest save file is loaded, the ID is recorded.
/// When this function is called, root's ID is compared to the recorded ID.
/// If they are different, calc the diff from the source)
///
/// Concurrent access is not supported. This function is synced by a mutex.
///
/// # Arguments
///
/// * `history_dir` - the directory which has hash directories which contains history files.
/// * `name` - arbitrary string to distinguish files. It's appended to the file name.
/// * 'root' - the object to save
/// * 'cache' - Cached data to make the process faster.
/// * 'options' - Options
///
/// The algorithm to generate diffs is described [here](https://github.com/dochy-ksti/dochy/blob/master/dochy_manual/src/sample_test/sample_code/history.md)
pub fn save_history_file<P : AsRef<Path>>(history_dir: P,
                             tag : Option<String>,
                             root : &RootObject,
                             cache : &mut DochyCache,
                             options : &HistoryOptions) -> FsResult<()> {
    let history_dir = history_dir.as_ref();
    let src = cache.current_src();

    let history_hash_dir = prepare_history_hash_dir(history_dir, src)?;
    fs_next(tag, root, cache, history_hash_dir, options)
}