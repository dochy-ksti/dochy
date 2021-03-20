use std::path::{Path, PathBuf};
use crate::imp::common::path::hash::hash_to_folder_name;

/// hash dir's path can be calculated with proj_dir(save_dir or history_dir)'s path and hash value
pub fn hash_dir_path<P : AsRef<Path>>(proj_dir: P, hash : u128) -> PathBuf{
    let name = hash_to_folder_name(hash);
    proj_dir.as_ref().join(&name)
}