use crate::fs::history::DochyCache;
use std::path::Path;
use crate::core::structs::RootObject;
use crate::fs::error::FsResult;
use crate::fs::imp::history::fs::start_new::start_new as fs_start_new;
use crate::fs::imp::history::file_hist::prepare_history_hash_dir::prepare_history_hash_dir;

pub fn start_new<P : AsRef<Path>>(
    history_dir : P,
    tag : Option<String>,
    diff_src : &RootObject,
    cache : &mut DochyCache,
    max_phase : usize) -> FsResult<()>{

    let history_dir  = history_dir.as_ref();
    let src = cache.current_src();

    let dir = prepare_history_hash_dir(history_dir, src)?;

    fs_start_new(tag, diff_src, cache, &dir, max_phase)
}

