use crate::error::FsResult;
use std::path::{ PathBuf};
use crate::imp::history::diff_and_cache::cacher::Cache;
use crate::imp::history::diff_and_cache::diff_src::DiffSrc;
use crate::imp::history::diff_and_cache::diff_value::DiffValue;
use crate::imp::history::diff_and_cache::open_diff_file_without_metadata::open_diff_file_without_metadata;
use rayon::{ThreadPool, ThreadPoolBuilder};
use std::collections::HashMap;

///traitの処理の中核である
pub(crate) fn accumulate_diff<V : DiffValue, S: DiffSrc<V>, C : Cache<V,S>>(
    paths: Vec<PathBuf>, cache : &mut C, max_phase : usize, caching : bool) -> FsResult<S>{
    let (mut item, paths) = cache.get_cache(paths, max_phase, caching)?;

    for path in &paths{
        let mut file = open_diff_file_without_metadata(path)?;
        let diff = V::read_value(&mut file)?;
        item.apply_diff(&diff)?;
    }

    Ok(item)
}

pub(crate) fn accumulate_diff_mt<V : DiffValue, S: DiffSrc<V>, C : Cache<V,S>>(
    paths: Vec<PathBuf>, cache : &mut C, max_phase : usize, caching : bool) -> FsResult<S>{
    use rayon::prelude::*;
    use std::sync::mpsc::{Sender, Receiver};
    use std::sync::mpsc;
    use std::thread;

    let (item, paths) = cache.get_cache(paths, max_phase, caching)?;
    let mut root = item.clone();
    let pool = ThreadPoolBuilder::new().build().unwrap();
    let (sender, receiver): (Sender<(usize,V::A)>, Receiver<(usize,V::A)>) = mpsc::channel();
    thread::spawn(move ||{
        for (i,path) in paths.iter().enumerate() {
            let mut file = open_diff_file_without_metadata(path)?;
            let v = V::read_value(&mut file)?;
            pool.spawn(move ||{
                let va = v.prepare(&item).unwrap();
                sender.send((i, va));
            });
        }
    });
    let mut map : HashMap<usize, V::A> = HashMap::new();
    let mut current_index = 0;
    for _ in 0..paths.len() {
        let (i,va) = receiver.recv().or(Err("preparing failed"))?;
        map.insert(i, va);
        apply_diff(&mut root, &mut current_index, &map);
    }
    Ok(root)
}

fn apply_diff<V : DiffValue, S: DiffSrc<V>>(root : &mut S,
                                            current_index : &mut usize,
                                            map : &HashMap<usize, V::A>){

}
