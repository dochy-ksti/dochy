use crate::imp::history::diff_and_cache::cacher::Cache;
use std::path::{PathBuf, Path};
use crate::error::FsResult;
use dochy_core::structs::RootObject;
use crate::imp::common::current_src::CurrentSrc;
use dochy_diff::apply_diff;
use crate::imp::history::diff_and_cache::dochy_diff::DochyDiff;
use crate::imp::history::diff_and_cache::open_diff_file_without_metadata::open_diff_file_without_metadata;
use std::collections::HashMap;
use dochy_archiver::ArchiveData;

//TODO: ここをやれ


pub struct DochyCache{
    current_src : CurrentSrc,
    src_root : Option<(RootObject, u128)>,
    phase_cache: HashMap<usize, (PathBuf, RootObject)>,
    max_phase:usize,
}

impl DochyCache{
    pub fn new(current_src : CurrentSrc, max_phase : usize) -> DochyCache{
        DochyCache{
            current_src,
            src_root : None,
            phase_cache : HashMap::new(),
            max_phase
        }
    }

    pub fn src_root(&mut self) -> (RootObject, u128){
        if self.src_root.is_none(){

        }
        if let Some((root, hash)) = self.src_root.as_ref(){
            return (root.clone(), *hash);
        }

        cache.current_src() {
            CurrentSrc::SrcDir(src_dir) => {
                let new = json_dir_to_root(src_dir, validation)?;
                let adjusted = adjust_versions(new, loaded, validation)?;
                Ok(adjusted)
            },
            CurrentSrc::ArchiveFile(current_archive) => {
                let new = load_archive(current_archive, validation)?;
                let adjusted = adjust_versions(new, loaded, validation)?;
                Ok(adjusted)
            }
        }
    }

    pub fn root_with_archive(&mut self) -> (RootObject, u128, ArchiveData){

    }

}