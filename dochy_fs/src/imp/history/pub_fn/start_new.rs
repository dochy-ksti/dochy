// use crate::history::DochyCache;
// use std::path::Path;
// use dochy_core::structs::RootObject;
// use crate::error::FsResult;
// use crate::imp::history::fs::start_new::start_new as fs_start_new;
// use crate::imp::history::file_hist::prepare_history_hash_dir::prepare_history_hash_dir;
//
// /// Calculates diff from the source file directly and save it.
// ///
// /// When a history file is loaded, and it doesn't directory derived from the last save file,
// /// calling next() is not sound.
// ///
// /// Since next() calculates diff from the last save file(basically), loading old file and calling next()
// /// can make an unnecessarily big diff file.
// ///
// /// The algorithm of constructing history files depends on the presumption that diffs are
// /// linearly accumulated, so using next() for such data can break the presumption,
// /// so using start_new is recommended.
// ///
// /// What max_phase is is described [here](https://github.com/dochy-ksti/dochy/blob/master/dochy_manual/src/sample_test/sample_code/history.md)
// pub fn start_new<P : AsRef<Path>>(
//     history_dir : P,
//     tag : Option<String>,
//     diff_src : &RootObject,
//     cache : &mut DochyCache,
//     max_phase : usize) -> FsResult<()>{
//
//     let history_dir  = history_dir.as_ref();
//     let src = cache.current_src();
//
//     let dir = prepare_history_hash_dir(history_dir, src)?;
//
//     fs_start_new(tag, diff_src, cache, &dir, max_phase)
// }
//
