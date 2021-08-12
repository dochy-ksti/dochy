use crate::imp::history::diff_and_cache::cacher::Cache;
use crate::test_simple_history::simple_diff::sd_diff::SdDiff;
use crate::test_simple_history::simple_diff::sd_data::SdData;
use std::path::PathBuf;
use crate::error::FsResult;

pub(crate) struct SdCache{
    size : Option<usize>
}

impl SdCache{
    pub(crate) fn new(size : Option<usize>) -> SdCache{ SdCache{ size } }
}

impl Cache<SdDiff, SdData> for SdCache{
    fn get_cache(&mut self, pathes: Vec<PathBuf>, _max_phase : usize, _caching : bool) -> FsResult<(SdData, Vec<PathBuf>)> {
        Ok((SdData::new(self.size), pathes))
    }

    fn set_cache(&mut self, _path: PathBuf, _item: SdData, _phase: usize) -> FsResult<()> {
        Ok(())
    }
}