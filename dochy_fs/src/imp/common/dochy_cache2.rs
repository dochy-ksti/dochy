use crate::imp::history::diff_and_cache::cacher::Cache;
use std::path::{PathBuf, Path};
use crate::error::FsResult;
use dochy_core::structs::RootObject;
use crate::imp::common::current_src::CurrentSrc;
use dochy_diff::apply_diff;
use crate::imp::history::diff_and_cache::dochy_diff::DochyDiff;
use crate::imp::history::diff_and_cache::open_diff_file_without_metadata::open_diff_file_without_metadata;

//TODO: ここをやれ


pub struct DochyCache{
    current_src : PathBuf,
    inner : Option<CacheInner>,
}

struct CacheInner{
    src_cache : RootObject,
    phase_cache: Vec<(PathBuf, RootObject)>,
    hash : u128,
}

impl DochyCache{

    /// The minimum constructor, which does create(current_src, true, true)
    pub fn new<P : AsRef<Path>>(current_src : P) -> DochyCache{

    }

    pub fn create_with_current_src(current_src : CurrentSrc) -> FsResult<DochyCache>{
        let root = current_src.create_root();
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
        if let Some((cached_path, root)) = self.phase_cache.as_ref().map(|a| a.as_ref()){
            if cached_path == path{
                return Ok(root.clone());
            }
        }
        let mut file = open_diff_file_without_metadata(&path)?;
        let mut root = src_root;
        apply_diff(&mut root, &mut file)?;
        self.phase_cache = Some(Box::new((path.to_path_buf(), root.clone())));
        return Ok(root)
    }

    pub fn current_src(&self) -> &CurrentSrc{ &self.current_src }
}