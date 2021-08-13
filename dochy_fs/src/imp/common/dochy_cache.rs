use std::path::{PathBuf};
use crate::error::FsResult;
use dochy_core::structs::RootObject;
use crate::imp::common::current_src::CurrentSrc;
use std::collections::{BTreeMap};
use dochy_archiver::{get_hash_and_metadata_from_dir};
use dochy_core::json_dir_to_root;
use crate::imp::common::archive::archive_opt::JSON_ARC_OPT;
use crate::imp::common::archive::load_archive::load_archive_and_hash;
use crate::imp::common::apply_items::{apply_items_st, apply_items_mt};


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

    /// Single threaded, caches except max_phase item
    pub fn apply_items_for_save(&mut self, paths: Vec<PathBuf>, max_phase : usize) -> FsResult<RootObject> {
        let first_len = paths.len();
        let (root,paths) = get_cached_item(self, paths, max_phase)?;
        let num_cached = paths.len() - first_len;
        remove_upper_phase_cache(&mut self.phase_cache, num_cached);
        let mut current_index = num_cached;

        //セーブは非同期で実行されるので、時間がかかってもかまうことはない、という発想
        apply_items_st(root, &paths, |r|{
            if current_index < max_phase{
                let path = &paths[current_index - num_cached];
                self.phase_cache.insert(current_index, (path.to_path_buf(), r.clone()));
                current_index += 1;
            }
        })
    }

    /// Multi-threaded, no cache
    pub fn apply_items_for_load(&mut self, paths: Vec<PathBuf>, max_phase : usize) -> FsResult<RootObject> {
        let (root,paths) = get_cached_item(self, paths, max_phase)?;
        //ロードではキャッシュを行わず、全力でただ開く。ロードが終わるまで処理が進まないことが想定されている
        apply_items_mt(root, paths,|_|{})
    }

    pub fn set_cache(&mut self, path : PathBuf, item: RootObject, phase: usize) {
        remove_upper_phase_cache(&mut self.phase_cache, phase);
        self.phase_cache.insert(phase, (path, item));
    }
}

/// removes phase <= index
fn remove_upper_phase_cache(cache : &mut BTreeMap<usize, (PathBuf, RootObject)>, phase : usize){
    loop{
        let index = if let Some((index, _)) = cache.iter().last(){ *index }
        else{
            break;
        };
        if phase <= index{
            cache.remove(&index);
        } else{
            break;
        }
    }
}

fn get_cached_item(cache : &mut DochyCache, paths: Vec<PathBuf>, max_phase : usize) -> FsResult<(RootObject, Vec<PathBuf>)> {
    if let Some(index) = get_phase_cache(&cache.phase_cache, &paths, max_phase){
        if index == max_phase{
            let (_,root) = cache.phase_cache.get(&index).unwrap();
            return Ok((root.clone(), Vec::new()));
        } else{
            let root = {
                let (_, root) = cache.phase_cache.get(&index).unwrap();
                root.clone()
            };
            let ans = paths.into_iter().skip(index+1).collect();
            return Ok((root, ans));
        }
    } else{
        Ok((cache.clone_src_root(), paths))
    }
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
