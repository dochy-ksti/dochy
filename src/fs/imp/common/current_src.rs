use std::path::{ PathBuf};
use crate::core::structs::RootObject;
use crate::fs::error::FsResult;
use crate::fs::imp::common::archive::load_archive::load_archive;

/// We always have an archive of json src files which corresponds to a save file.
/// if there's no change in json src files, we don't need current src.
/// if the src is changed, we need the archive and the current src
/// because we need to update the save data and make it compatible with the current src.
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
                Ok(crate::core::json_dir_to_rust(dir, false)?)
            },
            CurrentSrc::ArchiveFile(arc) =>{
                load_archive(arc, false)
            }
        }
    }
}