use crate::core::structs::RootObject;
use crate::diff::imp::write::write_root::write_root;
use crate::compaction::{encode_to_vec};
use crate::diff::diff_error::DiffError;
use crate::diff::imp::prepare::get_root_diff::get_root_diff;
use crate::compaction::kval_enum::KVal;

pub fn get_diff(from : &RootObject, to : &RootObject) -> Result<Vec<u8>, DiffError> {
    let kvals = get_kvals(from, to)?;
    Ok(encode_to_vec(&kvals)?)
}

pub fn get_kvals(from: &RootObject, to: &RootObject) ->Result<Vec<KVal>, DiffError> {
    let root_diff = get_root_diff(from, to)?;
    let kvals = write_root(root_diff)?;
    Ok(kvals)
}



