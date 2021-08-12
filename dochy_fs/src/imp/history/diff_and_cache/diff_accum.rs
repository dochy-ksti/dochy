use crate::imp::history::diff_and_cache::diff_value::DiffValue;
use crate::imp::history::diff_and_cache::diff_src::DiffSrc;
use crate::imp::history::diff_and_cache::cacher::Cache;
use std::path::PathBuf;

pub(crate) trait DiffAccum<V : DiffValue, S : DiffSrc<V>, C : Cache<V,S>>{
    fn accumulate_diff(paths : Vec<PathBuf>, cache : &mut C, max_phase : usize, caching : bool);
}