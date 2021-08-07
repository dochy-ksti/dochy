// use std::path::{Path};
// use dochy_core::structs::RootObject;
// use dochy_archiver::{read_archive_data};
// use crate::error::FsResult;
// use std::fs::File;
// use crate::imp::common::archive::find_or_make_and_read_archive::{make_archive_if_not_exist, BufOrPath};
// use crate::imp::common::archive::read_archive::read_archive;
//
// pub(crate) fn archive_archive(archive_path : &Path, proj_dir : &Path) -> FsResult<(RootObject, u128)>{
//     let mut file = File::open(archive_path)?;
//     let archive_data = read_archive_data(&mut file)?;
//     let hash = archive_data.meta().calc_hash()?;
//     make_archive_if_not_exist(proj_dir, hash, BufOrPath::ArchivePath(archive_path))?;
//     Ok((read_archive(&archive_data, false)?, hash))
// }