use crate::error::FsResult;
use std::path::{ PathBuf};
use crate::imp::history::diff_and_cache::cacher::Cache;
use crate::imp::history::diff_and_cache::diff_src::DiffSrc;
use crate::imp::history::diff_and_cache::diff_value::DiffValue;
use crate::imp::history::diff_and_cache::open_diff_file_without_metadata::open_diff_file_without_metadata;

///traitの処理の中核である
pub(crate) fn accumulate_diff<V : DiffValue, S: DiffSrc<V>, C : Cache<V, S>>(paths: Vec<PathBuf>, cache : &mut C) -> FsResult<S>{
    let (mut item, paths) = cache.get_cache(paths)?;

    for path in &paths{
        let mut file = open_diff_file_without_metadata(path)?;
        let diff = V::read_value(&mut file)?;
        item.apply_diff(&diff)?;
    }

    Ok(item)
}

