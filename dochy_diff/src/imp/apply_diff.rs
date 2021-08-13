use dochy_core::structs::{RootObject, MetaTable};
use crate::diff_error::DiffError;
use crate::imp::read::reader::Reader;
use crate::imp::read::read_root::read_root;
use crate::imp::apply::apply_root_diff::apply_root_diff;
use std::io::Read;
use dochy_compaction::enc_dec::decode::decode;
use crate::imp::structs_read::RootDiffR;
use dochy_compaction::kval_enum::KVal;

/// Applies diff data and restores the object
pub fn apply_diff<R : Read>(root : &mut RootObject, diff : &mut R) -> Result<(), DiffError>{
    let (diff, _size) = decode(diff)?;
    let mut reader = Reader::new(diff);
    let diff = read_root(&mut reader, root.meta_table())?;
    apply_root_diff(root, diff)?;
    Ok(())
}

pub fn get_root_diff_r(kvals : Vec<KVal>, meta_table : &MetaTable) -> Result<RootDiffR, DiffError>{
    let mut reader = Reader::new(kvals);
    Ok(read_root(&mut reader, meta_table)?)
}

pub fn apply_root_diff_r(root : &mut RootObject, diff : RootDiffR) -> Result<(), DiffError>{
    apply_root_diff(root, diff)
}