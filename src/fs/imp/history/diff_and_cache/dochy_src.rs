use crate::fs::imp::history::diff_and_cache::diff_src::DiffSrc;
use crate::core::structs::RootObject;
use crate::fs::error::FsResult;
use crate::fs::imp::history::diff_and_cache::dochy_diff::DochyDiff;


impl DiffSrc<DochyDiff> for RootObject{
    fn create_diff(&self, from: &Self) -> FsResult<DochyDiff> {
        let vec = crate::diff::get_diff(&from, self)?;
        Ok(DochyDiff::new(vec))
    }

    fn apply_diff(&mut self, diff: &DochyDiff) -> FsResult<()> {
        crate::diff::apply_diff(self, &mut diff.slice())?;
        Ok(())
    }
}

