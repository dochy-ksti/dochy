use dochy_core::structs::RustIdentity;
use std::io::{Write, Read};
use crate::history::FileNameProps;
use crate::error::{FsResult};
use dochy_compaction::kval_enum::KVal;
use dochy_compaction::basic_compaction::{comp_str, comp_int};

pub(crate) fn write<W : Write>(write : &mut W, id : &RustIdentity, newest_file : &FileNameProps) -> FsResult<()>{
    let mut vec : Vec<KVal> = vec![];
    vec.push(comp_str(newest_file.calc_filename()));
    vec.push(comp_int(id.random1() as i64));
    vec.push(comp_int(id.random2() as i64));
    dochy_compaction::encode(&vec, write)?;
    Ok(())
}

pub(crate) fn read<R : Read>(read : &mut R) -> FsResult<(RustIdentity, FileNameProps)>{
    let (v, _) = dochy_compaction::decode(read)?;
    let mut i = v.iter();
    let filename = i.next()?.as_string()?;
    let props = FileNameProps::from(&filename)?;

    let r1 = i.next()?.as_i64()? as u64;
    let r2 = i.next()?.as_i64()? as u64;

    Ok((RustIdentity::create(r1, r2), props))
}