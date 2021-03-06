use std::path::{ PathBuf};
use dochy_core::structs::RootObject;
use crate::error::FsResult;
use crate::imp::common::archive::load_archive::load_archive;

/// We always have an archive of json src files which corresponds to save files.
/// if there's no change in json src files, we don't need current src.
/// if the src is changed, we need the archived and the current src
/// because we need to update the save data and make it compatible with the current version.
/// The current src can be specified as an archive or a directory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CurrentSrc{
    SrcDir(PathBuf),
    ArchiveFile(PathBuf),
}

impl CurrentSrc{
    pub(crate) fn create_root(&self) -> FsResult<RootObject>{
        match self{
            CurrentSrc::SrcDir(dir) =>{
                Ok(dochy_core::json_dir_to_root(dir, false)?)
            },
            CurrentSrc::ArchiveFile(arc) =>{
                load_archive(arc, false)
            }
        }
    }
}