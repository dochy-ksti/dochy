use crate::imp::history::diff_and_cache::cacher::Cache;
use std::path::{PathBuf};
use crate::error::FsResult;
use dochy_core::structs::RootObject;
use crate::imp::history::diff_and_cache::dochy_diff::DochyDiff;
use crate::imp::common::dochy_cache::DochyCache;
use std::collections::BTreeMap;


impl Cache<DochyDiff, RootObject> for DochyCache {
    fn get_cache(&mut self, mut paths: Vec<PathBuf>, max_phase : usize) -> FsResult<(RootObject, Vec<PathBuf>)> {
        self.get_cache(paths, max_phase)
    }
}
