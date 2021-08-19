use std::path::{PathBuf, Path};
use crate::error::FsResult;
use std::io::Write;
use dochy_archiver::{create_archive_from_directory, get_hash_and_metadata_from_dir};
use crate::imp::common::archive::archive_opt::JSON_ARC_OPT;
use dochy_core::structs::RootObject;
use dochy_core::json_dir_to_root;
use crate::imp::common::archive::load_archive::load_archive_and_hash;

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
     pub fn create_root_and_hash(&self, validation : bool) -> FsResult<(RootObject, u128)>{
         match self{
             CurrentSrc::SrcDir(src_dir) => {
                 let root = json_dir_to_root(src_dir, validation)?;
                 let (hash, _meta) = get_hash_and_metadata_from_dir(src_dir, &JSON_ARC_OPT)?;
                 Ok((root, hash))
             },
             CurrentSrc::ArchiveFile(path) =>{
                 load_archive_and_hash(path, validation)
             }
         }
     }



    pub fn from_src_dir<P : AsRef<Path>>(src_dir : P) -> CurrentSrc{
        CurrentSrc::SrcDir(PathBuf::from(src_dir.as_ref()))
    }

    pub fn from_archive_file<P : AsRef<Path>>(archive_file_path : P) -> CurrentSrc{
        CurrentSrc::ArchiveFile(PathBuf::from(archive_file_path.as_ref()))
    }

    pub fn create_archive<W : Write>(&self, write : &mut W) -> FsResult<()>{
        match &self{
            CurrentSrc::SrcDir(src_dir) => {
                create_archive_from_directory(
                    src_dir,  write,
                    |_| false, &*JSON_ARC_OPT)?;
                Ok(())
            },
            CurrentSrc::ArchiveFile(path) =>{
                let mut file = std::fs::File::open(path)?;
                std::io::copy(&mut file, write)?;
                Ok(())
            }
        }
    }
}