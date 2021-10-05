use crate::error::CoreResult;
use crate::structs::RootObject;
use std::io::Read;
use crate::imp::json_to_rust::json_file_to_rust::json_file_to_rust;
use crate::imp::json_to_rust::archive_data_to_root::archive_data_to_root_with_hash;
use crate::imp::json_to_rust::validation::validate_root::validate_root;

pub fn archive_file_to_rust_with_hash<R : Read>(r : &mut R, validation : bool) -> CoreResult<(RootObject, u128)>{
    let data = dochy_archiver2::read_archive(json_file_to_rust, r)?;
    let (root, hash) = archive_data_to_root_with_hash(data)?;
    if validation{
        validate_root(&root, false)?;
    }
    Ok((root, hash))
}

pub fn archive_file_to_rust<R : Read>(r : &mut R, validation : bool) -> CoreResult<RootObject>{
    archive_file_to_rust_with_hash(r, validation).map(|(root,_)| root)
}