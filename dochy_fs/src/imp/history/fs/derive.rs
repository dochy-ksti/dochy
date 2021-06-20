use std::path::Path;
use crate::error::FsResult;
use crate::imp::history::fs::first::first;
use crate::imp::history::algo::phase_data::PhaseData;
use crate::imp::history::algo::calc_next_phase::calc_next_phase;
use crate::imp::history::fs::write_phase_file::write_phase_file;
use crate::imp::history::diff_and_cache::diff_src::DiffSrc;
use crate::imp::history::diff_and_cache::diff_value::DiffValue;
use crate::imp::history::diff_and_cache::accumulate_diff::accumulate_diff;
use crate::imp::history::diff_and_cache::cacher::Cache;
use crate::imp::history::algo::history_options::{HistoryOptions};
use crate::imp::history::file_hist::create_file_history::create_file_history;
use crate::history::FileNameProps;
use crate::imp::history::fs::derive_impl::derive_impl;


pub(crate) fn derive<
    V : DiffValue,
    S: DiffSrc<V>,
    C : Cache<V, S>,
    P : AsRef<Path>>(tag : Option<String>,
                     diff_src: &S,
                     cache : &mut C,
                     history_hash_dir: P,
                     from : &FileNameProps,
                     options: &HistoryOptions) -> FsResult<FileNameProps> {
    let history_hash_dir = history_hash_dir.as_ref();

    let history = create_file_history(history_hash_dir, Some(options.max_phase()))?;

    derive_impl(tag, diff_src, cache, history_hash_dir, &history, from, options)
}