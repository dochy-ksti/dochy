use std::path::Path;
use crate::fs::error::FsResult;
use crate::fs::imp::common::path::hash_dir_path::hash_dir_path;

///we can safely remove hash dirs.
pub fn remove_hash_dir<P : AsRef<Path>>(proj_dir : P, hash : u128) -> FsResult<()>{
    Ok(std::fs::remove_dir_all(hash_dir_path(proj_dir, hash))?)
}