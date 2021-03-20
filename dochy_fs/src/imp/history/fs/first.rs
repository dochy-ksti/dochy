use std::path::Path;
use crate::error::FsResult;
use crate::imp::history::fs::write_phase_a::write_phase_a;
use crate::imp::history::diff_and_cache::diff_src::DiffSrc;
use crate::imp::history::diff_and_cache::diff_value::DiffValue;
use crate::imp::history::diff_and_cache::cacher::Cache;

pub(crate) fn first<V : DiffValue, S : DiffSrc<V>, C : Cache<V,S>>(
    tag : Option<String>, diff_src : &S, cache : &mut C, history_hash_dir: &Path) -> FsResult<()>{
    write_phase_a(tag, 0, diff_src, cache, history_hash_dir)
}