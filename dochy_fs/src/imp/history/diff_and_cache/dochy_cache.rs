use crate::imp::history::diff_and_cache::cacher::Cache;
use std::path::{PathBuf, Path};
use crate::error::FsResult;
use dochy_core::structs::RootObject;
use crate::imp::common::current_src::CurrentSrc;
use dochy_diff::apply_diff;
use crate::imp::history::diff_and_cache::dochy_diff::DochyDiff;
use crate::imp::history::diff_and_cache::open_diff_file_without_metadata::open_diff_file_without_metadata;


/// Customizes how cache works
pub struct DochyCache{
    src_cache : Option<Box<RootObject>>,
    phase_a_cache : Option<Box<(PathBuf, RootObject)>>,
    current_src : CurrentSrc,
    cache_src : bool,
    cache_phase_a : bool,
}

impl Cache<DochyDiff, RootObject> for DochyCache{
    fn get_cache(&mut self, mut pathes: Vec<PathBuf>) -> FsResult<(RootObject, Vec<PathBuf>)> {
        let mut root = if self.cache_src == false{
            self.current_src.create_root()?
        } else{
            self.get_or_create_src()?
        };
        if pathes.len() == 0{
            return Ok((root, pathes));
        }
        let path = pathes.remove(0);
        if self.cache_phase_a == false{
            let mut file = open_diff_file_without_metadata(&path)?;

            apply_diff(&mut root, &mut file)?;
            return Ok((root, pathes));
        } else{
            let a = self.get_or_create_phase_a(root, &path)?;
            return Ok((a, pathes))
        }
    }
}

impl DochyCache{
    pub fn new(current_src : CurrentSrc, cache_src : bool, cache_phase_a : bool) -> DochyCache{
        DochyCache{
            src_cache: None,
            phase_a_cache: None,
            current_src,
            cache_src,
            cache_phase_a
        }
    }

    fn get_or_create_src(&mut self) -> FsResult<RootObject> {
        if let Some(root) = self.src_cache.as_ref().map(|a| a.as_ref()) {
            return Ok(root.clone());
        }
        let root = self.current_src.create_root()?;
        self.src_cache = Some(Box::new(root.clone()));
        Ok(root)
    }

    fn get_or_create_phase_a(&mut self, src_root: RootObject, path : &Path) -> FsResult<RootObject>{
        if let Some((cached_path, root)) = self.phase_a_cache.as_ref().map(|a| a.as_ref()){
            if cached_path == path{
                return Ok(root.clone());
            }
        }
        let mut file = open_diff_file_without_metadata(&path)?;
        let mut root = src_root;
        apply_diff(&mut root, &mut file)?;
        self.phase_a_cache = Some(Box::new((path.to_path_buf(), root.clone())));
        return Ok(root)
    }

    pub fn current_src(&self) -> &CurrentSrc{ &self.current_src }
}