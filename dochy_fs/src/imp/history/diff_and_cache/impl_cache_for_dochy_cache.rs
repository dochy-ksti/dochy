use crate::imp::history::diff_and_cache::cacher::Cache;
use std::path::{PathBuf};
use crate::error::FsResult;
use dochy_core::structs::RootObject;
use crate::imp::history::diff_and_cache::dochy_diff::DochyDiff;
use crate::imp::common::dochy_cache::DochyCache;
use std::collections::BTreeMap;


impl Cache<DochyDiff, RootObject> for DochyCache {
    fn get_cache(&mut self, mut pathes: Vec<PathBuf>, phase_max : usize) -> FsResult<(RootObject, Vec<PathBuf>)> {
        self.get_cache(pathes)
    }
}

fn get_phase_cache(cache : &BTreeMap<usize, (PathBuf, RootObject)>, vec : &Vec<PathBuf>) -> Option<Option<usize>>{
    let (_,(path, _)) = cache.iter().last()?;
    let last_path = vec.iter().last()?;
    if path == last_path{ return Some(None); }


    for i in (0..cache.len()).rev(){
        if let Some((c,_)) = cache.get(&i){
            if let Some(p) = vec.get(i){
                if c == p{ return Some(Some(i)); }
            }
        }
    }
    return None;

}
