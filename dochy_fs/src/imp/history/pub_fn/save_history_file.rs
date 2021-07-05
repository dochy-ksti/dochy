use crate::error::FsResult;
use std::path::Path;
use crate::history::HistoryOptions;
use dochy_core::structs::RootObject;
use crate::imp::history::file_hist::prepare_history_hash_dir::prepare_history_hash_dir;
use crate::imp::history::diff_and_cache::dochy_cache::DochyCache;
use crate::imp::history::fs::start_new::start_new as fs_start_new;
use crate::imp::history::latest_file_info::latest_file_info::{get_latest_file_info, set_latest_file_info, LatestFileInfo};
use crate::imp::history::fs::derive::derive;
use crate::imp::history::file_name::file_name_props::FileNameProps;
use crate::imp::history::file_hist::create_file_history::create_file_history;

/// calculates the diff from the latest save file(most of the time) and save the diff file.
/// If the 'root' is not derived from the latest save file, calculate diff from the source JSON5 and save it.
/// (RootObject has an ID and when the latest save file is loaded, the ID is recorded.
/// When this function is called, root's ID is compared to the recorded ID.
/// If they are different, calc the diff from the source)
///
/// Concurrent access to a history_dir is not supported.
/// Use synchronization if you want to access concurrently.
///
/// # Arguments
///
/// * `history_dir` - the directory which has hash directories which contains history files.
/// * `name` - arbitrary string to distinguish files. It's appended to the file name.
/// * 'root' - the object to save
/// * 'cache' - Cached data to make the process faster.
///
/// The algorithm to generate diffs is described [here](https://github.com/dochy-ksti/dochy/blob/master/dochy_manual/src/sample_test/sample_code/history.md)
pub fn save_history_file<P : AsRef<Path>, Op : AsRef<HistoryOptions>>(history_dir: P,
                             tag : Option<String>,
                             root : &RootObject,
                             cache : &mut DochyCache, opt : Op) -> FsResult<FileNameProps> {
    let history_dir = history_dir.as_ref();
    let opt = opt.as_ref();
    let src = cache.current_src();

    let (history_hash_dir, hash) = prepare_history_hash_dir(history_dir, src)?;


    let info = get_latest_file_info(history_dir, hash).clone();

    if let Some(info) = info.as_ref() {
        if root.id_ptr_eq(info.latest_root_id()) {
            let latest = if info.latest_base_file().phase() ==  opt.max_phase(){
                let history = create_file_history(&history_hash_dir, opt.max_phase(), opt.is_cumulative())?;
                history.
            } else{
                info.latest_base_file()
            }

            let latest = derive(tag, root, cache, &history_hash_dir, info.latest_base_file(), &HistoryOptions::new())?;
            set_latest_file_info(history_dir, hash, Some(LatestFileInfo::new(root.id(), latest.clone())));
            return Ok(latest);
        }
    }


    let opt = opt.as_ref();
    let latest = fs_start_new(tag, root, cache, &history_hash_dir, opt.max_phase(), opt.is_cumulative())?;
    set_latest_file_info(history_dir, hash, Some(LatestFileInfo::new(root.id(), latest.clone())));
    return Ok(latest);
}
