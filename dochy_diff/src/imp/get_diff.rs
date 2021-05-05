use dochy_core::structs::RootObject;
use crate::imp::write::write_root::write_root;
use crate::diff_error::DiffError;
use crate::imp::prepare::get_root_diff::get_root_diff;
use dochy_compaction::kval_enum::KVal;
use dochy_compaction::enc_dec::encode_to_vec::encode_to_vec;

/// Gets the difference from "from" to "to" in binary format.
///
/// When the diff binary is applied to a object with identical state of "from" object,
/// an object with identical state of "to" object will be constructed.
///
/// if 'to' is not derived from 'from', the diff binary can be invalid.
///
/// This system can't restore 'unmodified' state.
/// To calculate diff from newer data with a modified variable
/// to older data with the variable unmodified,
/// this system needs to restore 'unmodified' state, but it can't because of technical difficulties.
///
/// So you must make sure the "to" object is derived from "from" object.
pub fn get_diff(from : &RootObject, to : &RootObject) -> Result<Vec<u8>, DiffError> {
    let kvals = get_kvals(from, to)?;
    Ok(encode_to_vec(&kvals)?)
}

pub fn get_kvals(from: &RootObject, to: &RootObject) ->Result<Vec<KVal>, DiffError> {
    let root_diff = get_root_diff(from, to)?;
    let kvals = write_root(root_diff)?;
    Ok(kvals)
}



