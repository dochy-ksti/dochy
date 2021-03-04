use crate::core::structs::RootObject;
use crate::diff::diff_error::DiffError;
use crate::compaction::{decode};
use crate::diff::imp::read::reader::Reader;
use crate::diff::imp::read::read_root::read_root;
use crate::diff::imp::apply::apply_root_diff::apply_root_diff;
use std::io::Read;

pub fn apply_diff<R : Read>(root : &mut RootObject, diff : &mut R) -> Result<(), DiffError>{
    let (diff, _size) = decode(diff)?;
    let mut reader = Reader::new(diff);
    let diff = read_root(&mut reader, root.meta_table())?;
    apply_root_diff(root, diff)?;
    Ok(())
}