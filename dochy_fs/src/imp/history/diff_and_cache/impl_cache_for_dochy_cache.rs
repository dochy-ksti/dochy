use crate::imp::history::diff_and_cache::cache::Cache;
use std::path::{PathBuf};
use crate::error::FsResult;
use dochy_core::structs::RootObject;
use crate::imp::history::diff_and_cache::dochy_diff::DochyDiff;
use crate::imp::common::dochy_cache::DochyCache;


impl Cache<DochyDiff, RootObject> for DochyCache {
    fn apply_items(&mut self, paths: Vec<PathBuf>, max_phase: usize, caching: bool) -> FsResult<RootObject> {
        if caching {
            self.apply_items_for_save(paths, max_phase)
        } else{
            self.apply_items_for_load(paths, max_phase)
        }
    }

    fn set_cache(&mut self, path : PathBuf, item: RootObject, phase: usize) -> FsResult<()> {
        self.set_cache(path, item, phase);
        Ok(())
    }
}
