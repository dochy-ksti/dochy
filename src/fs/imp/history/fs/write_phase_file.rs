use std::path::Path;
use crate::fs::error::FsResult;
use crate::fs::imp::history::algo::phase_data::PhaseData;
use std::io::Write;

pub(crate) fn write_phase_file(data : &PhaseData, file_path : &Path, vec : &[u8]) -> FsResult<()>{
    let data_encoded = data.encode();
    let mut file = std::fs::File::create(file_path)?;
    crate::compaction::encode(&data_encoded, &mut file)?;
    //let mut vec = vec;
    file.write_all(vec)?;
    Ok(())
}