use std::path::{PathBuf};
use crate::error::FsResult;
use crate::imp::history::diff_and_cache::diff_src::DiffSrc;
use crate::imp::history::diff_and_cache::diff_value::DiffValue;

pub(crate) trait Cache<V : DiffValue, S : DiffSrc<V>>{

    /// diff ファイル適用済みのデータがキャッシュされている場合、そのデータを返し、
    /// さらにそこからapplyすべきdiffファイルのパスのリストも返す
    ///
    /// caching : 全部計算してキャッシュするか、自分がもってるキャッシュだけ返すか。どっちでもよい。trueでもfalseでも同じ処理でもよい
    fn apply_items(&mut self, paths : Vec<PathBuf>, max_phase : usize, caching : bool) -> FsResult<S>;
    fn set_cache(&mut self, path : PathBuf, item : S, phase : usize) -> FsResult<()>;
}