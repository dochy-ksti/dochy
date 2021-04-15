use dochy_core::structs::RootObject;
use crate::diff_error::DiffError;
use crate::imp::read::reader::Reader;
use crate::imp::read::read_root::read_root;
use crate::imp::apply::apply_root_diff::apply_root_diff;
use std::io::Read;
use dochy_compaction::enc_dec::decode::decode;

/// Applies diff data and restores the object
pub fn apply_diff<R : Read>(root : &mut RootObject, diff : &mut R) -> Result<(), DiffError>{
    let (diff, _size) = decode(diff)?;
    let mut reader = Reader::new(diff);
    let diff = read_root(&mut reader, root.meta_table())?;
    apply_root_diff(root, diff)?;
    Ok(())
}