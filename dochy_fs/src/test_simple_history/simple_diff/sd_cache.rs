use crate::imp::history::diff_and_cache::cache::Cache;
use crate::test_simple_history::simple_diff::sd_diff::SdDiff;
use crate::test_simple_history::simple_diff::sd_data::SdData;
use std::path::PathBuf;
use crate::error::FsResult;
use crate::imp::history::diff_and_cache::accumulate_diff::accumulate_diff;

pub(crate) struct SdCache{
    size : Option<usize>
}

impl SdCache{
    pub(crate) fn new(size : Option<usize>) -> SdCache{ SdCache{ size } }
}

impl Cache<SdDiff, SdData> for SdCache{
    fn apply_items(&mut self, paths: Vec<PathBuf>, max_phase: usize, caching: bool) -> FsResult<SdData> {
        let r = SdData::new(self.size);
        accumulate_diff(r, paths, max_phase, caching)
    }

    fn set_cache(&mut self, _path: PathBuf, _item: SdData, _phase: usize) -> FsResult<()> {
        Ok(())
    }
}