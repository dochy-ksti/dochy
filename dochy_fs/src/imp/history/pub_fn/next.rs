// use crate::error::FsResult;
// use std::path::Path;
// use crate::history::HistoryOptions;
// use dochy_core::structs::RootObject;
// use crate::imp::history::fs::next::next as fs_next;
// use crate::imp::history::file_hist::prepare_history_hash_dir::prepare_history_hash_dir;
// use crate::imp::history::diff_and_cache::dochy_cache::DochyCache;
//
// /// calculates the diff from the latest save file(most of the time) and save the diff file.
// /// If the 'root' is not derived from the latest save file, use start_new.
// ///
// /// # Arguments
// ///
// /// * `history_dir` - the directory to save history files and directories which contains the files.
// /// * `name` - arbitrary string to distinguish files. It's appended to the file name.
// /// * 'root' - the object to save
// /// * 'cache' - Cache to make the process faster.
// /// * 'options' - Options
// ///
// /// The algorithm to generate diffs is described [here](https://github.com/dochy-ksti/dochy/blob/master/dochy_manual/src/sample_test/sample_code/history.md)
// pub fn next<P : AsRef<Path>>(history_dir: P,
//                              tag : Option<String>,
//                              root : &RootObject,
//                              cache : &mut DochyCache,
//                              options : &HistoryOptions) -> FsResult<()> {
//     let history_dir = history_dir.as_ref();
//     let src = cache.current_src();
//
//     let history_hash_dir = prepare_history_hash_dir(history_dir, src)?;
//     fs_next(tag, root, cache, history_hash_dir, options)
// }
