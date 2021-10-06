

use crate::error::CoreResult;

use crate::imp::structs::root_obj::RootObject;
use std::path::Path;
use dochy_archiver2::read_archive_data_from_directory;
use crate::JSON_ARC_OPT;
use crate::imp::json_to_rust::archive_data_to_root::archive_data_to_root_with_hash;
use crate::imp::json_to_rust::json_file_to_rust::json_file_to_rust;
use crate::imp::json_to_rust::validation::validate_root::validate_root;
use crate::imp::json_to_rust::roots::archive_data_to_root::archive_data_to_root_with_hash;

/// Converts Dochy source files to RootObject
/// Does extra checks when validation=true
pub fn json_dir_to_root_with_hash<P : AsRef<Path>>(dir_path : P, validation : bool) -> CoreResult<(RootObject, u128)> {
    let archive = read_archive_data_from_directory(dir_path, &JSON_ARC_OPT, json_file_to_rust)?;
    let (root, hash) = archive_data_to_root_with_hash(archive)?;
    if validation{
        validate_root(&root, false)?;
    }
    return Ok((root, hash));
}

pub fn json_dir_to_root<P : AsRef<Path>>(dir_path : P, validation : bool) -> CoreResult<RootObject> {
    json_dir_to_root_with_hash(dir_path, validation).map(|(root,_)| root)
}

