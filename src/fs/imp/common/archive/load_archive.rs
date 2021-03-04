use crate::fs::error::FsResult;
use crate::core::structs::RootObject;
use std::path::Path;
use std::fs::File;
use crate::archiver::read_archive_data;
use crate::fs::imp::common::archive::read_archive::read_archive;

pub fn load_archive<P : AsRef<Path>>(archive_path : P, validation : bool) -> FsResult<RootObject>{
    let mut file = File::open(archive_path)?;
    let archive = read_archive_data(&mut file)?;
    Ok(read_archive(&archive, validation)?)
}