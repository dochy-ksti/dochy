use crate::imp::history::diff_and_cache::diff_src::DiffSrc;
use dochy_core::structs::{RootObject};
use crate::error::FsResult;
use crate::imp::history::diff_and_cache::dochy_diff::DochyDiff;


impl DiffSrc<DochyDiff> for RootObject{

    fn create_diff(&self, from: &Self) -> FsResult<DochyDiff> {
        let vec = dochy_diff::get_diff(&from, self)?;
        Ok(DochyDiff::new(vec))
    }

    fn apply_diff(&mut self, diff: DochyDiff) -> FsResult<()> {
        dochy_diff::apply_diff(self, &mut diff.slice())?;
        Ok(())
    }
}

