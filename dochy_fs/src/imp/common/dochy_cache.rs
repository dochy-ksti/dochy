use crate::imp::history::diff_and_cache::cacher::Cache;
use std::path::{PathBuf, Path};
use crate::error::FsResult;
use dochy_core::structs::RootObject;
use crate::imp::common::current_src::CurrentSrc;
use dochy_diff::apply_diff;
use crate::imp::history::diff_and_cache::dochy_diff::DochyDiff;
use crate::imp::history::diff_and_cache::open_diff_file_without_metadata::open_diff_file_without_metadata;
use std::collections::HashMap;
use dochy_archiver::{ArchiveData, get_hash_and_metadata_from_dir, create_archive_from_directory};
use dochy_core::json_dir_to_root;
use crate::imp::common::archive::archive_opt::JSON_ARC_OPT;
use crate::common::load_archive;
use crate::imp::common::archive::load_archive::load_archive_and_hash;
use std::io::Write;

//TODO: ここをやれ


pub struct DochyCache{
    current_src : CurrentSrc,
    root : RootObject,
    hash : u128,
    phase_cache: HashMap<usize, (PathBuf, RootObject)>,
    max_phase:usize,
}

impl DochyCache{
    pub fn create(current_src : CurrentSrc, max_phase : usize, validation : bool) -> FsResult<DochyCache>{
        let (root, hash) = match &current_src{
            CurrentSrc::SrcDir(src_dir) => {
                let root = json_dir_to_root(src_dir, validation)?;
                let (hash, _meta) = get_hash_and_metadata_from_dir(src_dir, &JSON_ARC_OPT)?;
                (root, hash)
            },
            CurrentSrc::ArchiveFile(path) =>{
                load_archive_and_hash(path, validation)?
            }
        };
        Ok(DochyCache{
            current_src, root, hash,
            phase_cache : HashMap::new(),
            max_phase
        })
    }

    pub fn new(current_src : CurrentSrc, max_phase : usize) -> FsResult<DochyCache>{
        create(current_src, max_phase, false)
    }

    pub fn create_archive<W : Write>(&self, write : &mut W) -> FsResult<()>{
        match &self.current_src{
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