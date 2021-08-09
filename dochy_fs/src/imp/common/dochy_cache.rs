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

    pub fn get_cache(&mut self, mut paths: Vec<PathBuf>, max_phase : usize) -> FsResult<(RootObject, Vec<PathBuf>)> {
        if let Some(op) = get_phase_cache(&self.phase_cache, &paths){

        } else{}

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

fn get_phase_cache(cache : &BTreeMap<usize, (PathBuf, RootObject)>, vec : &Vec<PathBuf>, max_phase : usize) -> Option<usize>{
    let (index,(path, _)) = cache.iter().last()?;
    let last_path = vec.iter().last()?;
    if path == last_path {
        return Some(*index);
    }


    for i in (0..max_phase).rev(){
        if let Some((c,_)) = cache.get(&i){
            if let Some(p) = vec.get(i){
                if c == p{ return Some(i); }
            }
        }
    }
    return None;

}
