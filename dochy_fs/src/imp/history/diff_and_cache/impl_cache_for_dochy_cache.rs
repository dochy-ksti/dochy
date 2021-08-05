// use crate::imp::history::diff_and_cache::cacher::Cache;
// use std::path::{PathBuf, Path};
// use crate::error::FsResult;
// use dochy_core::structs::RootObject;
// use crate::imp::common::current_src::CurrentSrc;
// use dochy_diff::apply_diff;
// use crate::imp::history::diff_and_cache::dochy_diff::DochyDiff;
// use crate::imp::history::diff_and_cache::open_diff_file_without_metadata::open_diff_file_without_metadata;
// use crate::imp::common::dochy_cache::DochyCache;
//
//
// impl Cache<DochyDiff, RootObject> for DochyCache{
//     fn get_cache(&mut self, mut pathes: Vec<PathBuf>) -> FsResult<(RootObject, Vec<PathBuf>)> {
//         // let mut root = if self.cache_src == false{
//         //     self.current_src.create_root()?
//         // } else{
//         //     self.get_or_create_src()?
//         // };
//         // if pathes.len() == 0{
//         //     return Ok((root, pathes));
//         // }
//         // let path = pathes.remove(0);
//         // if self.cache_phase_a == false{
//         //     let mut file = open_diff_file_without_metadata(&path)?;
//         //
//         //     apply_diff(&mut root, &mut file)?;
//         //     return Ok((root, pathes));
//         // } else{
//         //     let a = self.get_or_create_phase_a(root, &path)?;
//         //     return Ok((a, pathes))
//         // }
//         unimplemented!()
//     }
// }
