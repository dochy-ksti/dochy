use std::path::Path;
use crate::error::FsResult;
use crate::imp::history::fs::first::first;
use crate::imp::history::algo::phase_data::PhaseData;
use crate::imp::history::algo::calc_next_phase::calc_next_phase;
use crate::imp::history::fs::write_phase_file::write_phase_file;
use crate::imp::history::file_hist::ancestors::{Ancestors};
use crate::imp::history::diff_and_cache::diff_src::DiffSrc;
use crate::imp::history::diff_and_cache::diff_value::DiffValue;
use crate::imp::history::diff_and_cache::accumulate_diff::accumulate_diff;
use crate::imp::history::diff_and_cache::cacher::Cache;
use crate::imp::history::algo::history_options::{HistoryOptions};
use crate::imp::history::file_hist::create_file_history::create_file_history;
use crate::history::FileNameProps;
use crate::imp::history::file_hist::file_history::FileHistory;


pub(crate) fn derive_impl<
    V : DiffValue,
    S: DiffSrc<V>,
    C : Cache<V, S>,
    P : AsRef<Path>>(tag : Option<String>,
                     diff_src: &S,
                     cache : &mut C,
                     history_hash_dir: P,
                     history : &FileHistory,
                     from : &FileNameProps,
                     options: &HistoryOptions) -> FsResult<FileNameProps> {
    let from_file_path = history_hash_dir.as_ref().join(from.calc_filename());

    let mut file = std::fs::File::open(&from_file_path)?;
    let (decoded, _) = dochy_compaction::enc_dec::decode::decode(&mut file)?;
    let mut data = PhaseData::decode(&decoded)?;
    let next_phase = calc_next_phase(&data, options);
    let newest_ctl = history.get_newest_prop().control();
    let next_ctl = if newest_ctl == from.control(){ newest_ctl } else{ newest_ctl + 1 };
    let next_prop = from.create_next_phase_props(next_ctl, tag, next_phase)?;

    let ancestors = Ancestors::create(
        &history, &next_prop, options.max_phase(), options.is_cumulative())? ;

    let composed = accumulate_diff(ancestors.calc_paths(history_hash_dir), cache)?;
    let diff = diff_src.create_diff(&composed)?;

    let mut vec: Vec<u8> = vec![];
    diff.write_value(&mut vec)?;

    //ファイルに書き込む前に先にlenを求める必要がある
    data.pop_and_push(next_phase, vec.len() as u64);

    let next_file_path = history_hash_dir.join(&next_prop.calc_filename());

    write_phase_file(&data, &next_file_path, &vec)?;

    Ok(next_prop)
}