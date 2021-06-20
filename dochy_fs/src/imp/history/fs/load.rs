use std::path::Path;
use crate::error::FsResult;
use crate::imp::history::file_hist::file_history::FileHistory;
use crate::imp::history::diff_and_cache::diff_src::DiffSrc;
use crate::imp::history::diff_and_cache::diff_value::DiffValue;
use crate::imp::history::diff_and_cache::accumulate_diff::accumulate_diff;
use crate::imp::history::diff_and_cache::cacher::Cache;
use crate::imp::history::algo::history_options::{HistoryOptions};
use crate::imp::history::file_name::analyze_file_name::analyze_file_name;


pub(crate) fn load<
    V : DiffValue,
    S : DiffSrc<V>,
    C : Cache<V, S>,
    P : AsRef<Path>>(diff_file_path: P,
                     history : &FileHistory,
                     cache : &mut C,
                     op : &HistoryOptions) -> FsResult<S> {
    let path = diff_file_path.as_ref();
    let dir_path = path.parent()?;
    let filename = path.file_name()?.to_string_lossy().to_string();
    let analyzed = analyze_file_name(&filename, Some(op.max_phase()))
        .ok_or_else(|| format!("invalid file name {}", &filename))?;

    unimplemented!()
    // let ancestors = Ancestors::create(history,&analyzed, op.max_phase(), op.is_cumulative())?;
    // let mut paths = ancestors.calc_paths(dir_path);
    // paths.push(path.to_path_buf());
    // Ok(accumulate_diff(paths, cache)?)
}