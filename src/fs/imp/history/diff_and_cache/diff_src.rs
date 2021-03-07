use crate::fs::error::FsResult;
use crate::fs::imp::history::diff_and_cache::diff_value::DiffValue;

pub(crate) trait DiffSrc<V : DiffValue>{
    fn create_diff(&self, from: &Self) -> FsResult<V>;
    fn apply_diff(&mut self, diff : &V) -> FsResult<()>;
}