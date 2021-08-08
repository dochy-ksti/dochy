use std::path::{PathBuf};
use crate::error::FsResult;
use dochy_core::structs::RootObject;
use crate::imp::common::current_src::CurrentSrc;
use std::collections::{HashMap, BTreeMap};
use dochy_archiver::{get_hash_and_metadata_from_dir};
use dochy_core::json_dir_to_root;
use crate::imp::common::archive::archive_opt::JSON_ARC_OPT;
use crate::imp::common::archive::load_archive::load_archive_and_hash;



pub struct DochyCache{
    current_src : CurrentSrc,
    src_root : RootObject,
    hash : u128,
    pub(crate) phase_cache: BTreeMap<usize, (PathBuf, RootObject)>,
}

impl DochyCache{
    pub fn create(current_src : CurrentSrc, validation : bool) -> FsResult<DochyCache>{
        let (src_root, hash) = match &current_src{
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
            current_src, src_root, hash,
            phase_cache : BTreeMap::new(),
        })
    }

    pub fn new(current_src : CurrentSrc) -> FsResult<DochyCache>{
        Self::create(current_src, false)
    }

    pub fn current_src(&self) -> &CurrentSrc{ &self.current_src }
    pub fn hash(&self) -> u128{ self.hash }

    pub fn clone_src_root(&self) -> RootObject{
        self.src_root.clone()
    }

    pub fn get_cache(&mut self, mut pathes: Vec<PathBuf>) -> FsResult<(RootObject, Vec<PathBuf>)> {
        // let mut root = if self.cache_src == false{
        //     self.current_src.create_root()?
        // } else{
        //     self.get_or_create_src()?
        // };
        // if pathes.len() == 0{
        //     return Ok((root, pathes));
        // }
        // let path = pathes.remove(0);
        // if self.cache_phase_a == false{
        //     let mut file = open_diff_file_without_metadata(&path)?;
        //
        //     apply_diff(&mut root, &mut file)?;
        //     return Ok((root, pathes));
        // } else{
        //     let a = self.get_or_create_phase_a(root, &path)?;
        //     return Ok((a, pathes))
        // }
        unimplemented!()
    }
}