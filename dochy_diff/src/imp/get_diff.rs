use dochy_core::structs::RootObject;
use crate::imp::write::write_root::write_root;
use crate::diff_error::DiffError;
use crate::imp::prepare::get_root_diff::get_root_diff;
use dochy_compaction::kval_enum::KVal;
use dochy_compaction::enc_dec::encode_to_vec::encode_to_vec;

/// Gets the difference from "from" to "to" in binary format.
pub fn get_diff(from : &RootObject, to : &RootObject) -> Result<Vec<u8>, DiffError> {
    let kvals = get_kvals(from, to)?;
    Ok(encode_to_vec(&kvals)?)
}

pub fn get_kvals(from: &RootObject, to: &RootObject) ->Result<Vec<KVal>, DiffError> {
    let root_diff = get_root_diff(from, to)?;
    let kvals = write_root(root_diff)?;
    Ok(kvals)
}



