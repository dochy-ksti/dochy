use std::path::Path;
use crate::error::FsResult;
use crate::imp::history::fs::write_phase_0::write_phase_0;
use crate::imp::history::diff_and_cache::diff_src::DiffSrc;
use crate::imp::history::diff_and_cache::diff_value::DiffValue;
use crate::imp::history::diff_and_cache::cache::Cache;
use crate::imp::history::file_name::file_name_props::FileNameProps;

pub(crate) fn first<V : DiffValue, S : DiffSrc<V>, C : Cache<V,S>>(
    tag : Option<String>, diff_src : &S, cache : &mut C, max_phase : usize, history_hash_dir: &Path)
    -> FsResult<FileNameProps>{
    write_phase_0(tag, 0, diff_src, cache, max_phase, history_hash_dir)
}