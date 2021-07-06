use crate::imp::history::diff_and_cache::cacher::Cache;
use crate::test_simple_history::simple_diff::sd_diff::SdDiff;
use crate::test_simple_history::simple_diff::sd_data::SdData;
use std::path::PathBuf;
use crate::error::FsResult;

pub(crate) struct SdCache{}

impl SdCache{
    pub(crate) fn new() -> SdCache{ SdCache{} }
}

impl Cache<SdDiff, SdData> for SdCache{
    fn get_cache(&mut self, pathes: Vec<PathBuf>) -> FsResult<(SdData, Vec<PathBuf>)> {
        Ok((SdData::new(None), pathes))
    }
}