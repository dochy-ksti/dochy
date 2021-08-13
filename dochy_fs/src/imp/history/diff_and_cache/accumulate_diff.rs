use crate::error::FsResult;
use std::path::{ PathBuf};
use crate::imp::history::diff_and_cache::diff_src::DiffSrc;
use crate::imp::history::diff_and_cache::diff_value::DiffValue;
use crate::imp::history::diff_and_cache::open_diff_file_without_metadata::open_diff_file_without_metadata;

///traitの処理の中核である
pub(crate) fn accumulate_diff<V : DiffValue, S: DiffSrc<V>>(
    mut item : S, paths: Vec<PathBuf>, max_phase : usize, caching : bool) -> FsResult<S>{

    for path in &paths{
        let mut file = open_diff_file_without_metadata(path)?;
        let diff = V::read_value(&mut file)?;
        item.apply_diff(diff)?;
    }

    Ok(item)
}
