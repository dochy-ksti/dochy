use std::path::Path;
use crate::fs::error::FsResult;
use crate::fs::imp::history::file_name::calc_filename::calc_filename;
use crate::fs::imp::history::algo::phase_data::PhaseData;
use crate::fs::imp::history::fs::write_phase_file::write_phase_file;
use crate::fs::imp::history::diff_and_cache::diff_src::DiffSrc;
use crate::fs::imp::history::diff_and_cache::diff_value::DiffValue;
use crate::fs::imp::history::diff_and_cache::cacher::Cache;

pub(crate) fn write_phase_a<V : DiffValue, S: DiffSrc<V>, C : Cache<V,S>>(
    tag : Option<String>,
    control : u32,
    diff_src: &S,
    cache : &mut C,
    history_hash_dir: &Path) -> FsResult<()>{

    let file_name = calc_filename(tag.as_ref().map(|s| s.as_str()), control, &[0]);
    let file_path = history_hash_dir.join(file_name);

    let (initial, _) = cache.get_cache(vec![])?;

    let diff = diff_src.create_diff(&initial)?;
    let mut vec : Vec<u8> = vec![];
    diff.write_value(&mut vec)?;
    let data = PhaseData::new(vec.len() as u64);

    write_phase_file(&data, &file_path, &vec)
}