use std::path::{PathBuf};
use crate::error::FsResult;
use crate::imp::history::diff_and_cache::diff_src::DiffSrc;
use crate::imp::history::diff_and_cache::diff_value::DiffValue;

pub(crate) trait Cache<V, CachedItem> where
    V : DiffValue,
    CachedItem : DiffSrc<V>{

    /// diff ファイル適用済みのデータがキャッシュされている場合、そのデータを返し、
    /// さらにそこからapplyすべきdiffファイルのパスのリストも返す
    fn get_cache(&mut self, pathes : Vec<PathBuf>, max_phase : usize) -> FsResult<(CachedItem, Vec<PathBuf>)>;
}