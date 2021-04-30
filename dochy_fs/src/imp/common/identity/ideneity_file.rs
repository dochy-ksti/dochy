use dochy_core::structs::RustIdentity;
use std::io::Write;
use crate::history::FileNameProps;
use crate::error::{FsResult};
use dochy_compaction::kval_enum::KVal;

pub(crate) fn write<W : Write>(write : &mut W, id : &RustIdentity, newest_file : &FileNameProps) -> FsResult<()>{
    let mut vec : Vec<KVal> = vec![];
    aa

    dochy_compaction::encode(&vec, write)?;
    Ok(())
}