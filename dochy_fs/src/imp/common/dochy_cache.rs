use std::path::{PathBuf};
use crate::error::FsResult;
use dochy_core::structs::RootObject;
use crate::imp::common::current_src::CurrentSrc;
use std::collections::{BTreeMap};
use dochy_archiver::{get_hash_and_metadata_from_dir};
use dochy_core::json_dir_to_root;
use crate::imp::common::archive::archive_opt::JSON_ARC_OPT;
use crate::imp::common::archive::load_archive::load_archive_and_hash;
use crate::imp::history::diff_and_cache::open_diff_file_without_metadata::open_diff_file_without_metadata;
use dochy_diff::apply_diff;


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

    pub fn get_cache(&mut self, paths: Vec<PathBuf>, max_phase : usize) -> FsResult<(RootObject, Vec<PathBuf>)> {

        if let Some(index) = get_phase_cache(&self.phase_cache, &paths, max_phase){
            if index == max_phase{
                let (_,root) = self.phase_cache.get(&index).unwrap();
                return Ok((root.clone(), Vec::new()));
            } else{
                let root = {
                    let (_, root) = self.phase_cache.get(&index).unwrap();
                    root.clone()
                };
                let r = calc_diffs_and_cache(&mut self.phase_cache, root, &paths, max_phase, index+1)?;
                return Ok((r, Vec::new()))
            }
        } else{
            let root = self.clone_src_root();
            let r = calc_diffs_and_cache(&mut self.phase_cache, root, &paths, max_phase, 0)?;
            return Ok((r, Vec::new()))
        }
    }

    pub fn set_cache(&mut self, path : PathBuf, item: RootObject, phase: usize) -> FsResult<()> {
        loop{
            let index = if let Some((index, _)) = self.phase_cache.iter().last(){ *index }
            else{
                break;
            };
            if phase < index{
                self.phase_cache.remove(&index);
            } else{
                break;
            }
        }
        self.phase_cache.insert(phase, (path, item));
        Ok(())
    }
}

fn calc_diffs_and_cache(cache : &mut BTreeMap<usize, (PathBuf, RootObject)>,
                        mut root : RootObject,
                        paths : &[PathBuf],
                        max_phase : usize,
                        current_index : usize) -> FsResult<RootObject> {
    for i in current_index..max_phase {
        let path = if let Some(path) = paths.get(i){ path } else {
            return Ok(root);
        };
        let mut file = open_diff_file_without_metadata(&path)?;

        apply_diff(&mut root, &mut file)?;
        cache.insert(i, (path.to_path_buf(), root.clone()));
    }
    if paths.len() == max_phase{
        return Ok(root);
    }
    for i in max_phase..paths.len(){
        let path = &paths[i];
        let mut file = open_diff_file_without_metadata(&path)?;

        apply_diff(&mut root, &mut file)?;

        if i == paths.len() - 1{
            cache.insert(max_phase,(path.to_path_buf(), root.clone()));
            return Ok(root);
        }
    }
    unreachable!()
}

fn get_phase_cache(cache : &BTreeMap<usize, (PathBuf, RootObject)>, vec : &Vec<PathBuf>, max_phase : usize) -> Option<usize>{
    let (index,(path, _)) = cache.iter().last()?;
    let last_path = vec.iter().last()?;
    if path == last_path {
        //多分このパターンしかないと思うんだよね。
        //ただの計算のショートカットだから完全網羅しなくてもいいはず・・・
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
