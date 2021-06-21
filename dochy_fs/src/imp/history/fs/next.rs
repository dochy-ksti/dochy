use std::path::Path;
use crate::error::FsResult;
use crate::imp::history::fs::first::first;
use crate::imp::history::diff_and_cache::diff_src::DiffSrc;
use crate::imp::history::diff_and_cache::diff_value::DiffValue;
use crate::imp::history::diff_and_cache::cacher::Cache;
use crate::imp::history::algo::history_options::{HistoryOptions};
use crate::imp::history::file_hist::create_file_history::create_file_history;
use crate::imp::history::file_name::file_name_props::FileNameProps;
use crate::imp::history::fs::derive_impl::derive_impl;


pub(crate) fn next<
    V : DiffValue,
    S: DiffSrc<V>,
    C : Cache<V, S>,
    P : AsRef<Path>>(tag : Option<String>,
                     diff_src: &S,
                     cache : &mut C,
                     history_hash_dir: P,
                     options: &HistoryOptions) -> FsResult<FileNameProps> {

    let history_hash_dir = history_hash_dir.as_ref();

    let history = create_file_history(history_hash_dir, options.max_phase(), options.is_cumulative())?;
    let newest_prop = if let Some(prop) = history.get_newest_prop() {
         prop
    } else {
         return first(tag, diff_src, cache, history_hash_dir);
    };

    derive_impl(tag, diff_src, cache, history_hash_dir, &history, newest_prop, options)
    //
    // //dbg!(&newest_prop);
    // let newest_file_path = history_hash_dir.join(newest_prop.calc_filename());
    // //dbg!(&newest_file_path);
    // let mut file = std::fs::File::open(&newest_file_path)?;
    // let (decoded, _) = dochy_compaction::enc_dec::decode::decode(&mut file)?;
    // let mut data = PhaseData::decode(&decoded)?;
    // let next_phase = calc_next_phase(&data, options);
    // //dbg!(next_phase);
    // let next_prop = newest_prop.create_next_phase_props(newest_prop.control(), tag, next_phase)?;
    // //dbg!(&next_prop);
    // //let ancestors = Ancestors::create(&history, &next_prop, max_phase, cumulative, dir_path.into());
    // let ancestors = Ancestors::create(
    //     &history, &next_prop, options.max_phase(), options.is_cumulative())? ;
    //
    // let composed = accumulate_diff(ancestors.calc_paths(history_hash_dir), cache)?;
    // let diff = diff_src.create_diff(&composed)?;
    //
    // let mut vec: Vec<u8> = vec![];
    // diff.write_value(&mut vec)?;
    //
    // //ファイルに書き込む前に先にlenを求める必要がある
    // data.pop_and_push(next_phase, vec.len() as u64);
    //
    // let next_file_path = history_hash_dir.join(&next_prop.calc_filename());
    //
    // write_phase_file(&data, &next_file_path, &vec)?;
    //
    // Ok(next_prop)
}