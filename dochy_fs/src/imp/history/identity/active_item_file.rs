use dochy_core::structs::RustIdentity;
use std::io::{Write, Read};
use crate::error::{FsResult};
use dochy_compaction::kval_enum::KVal;
use dochy_compaction::basic_compaction::{comp_int};
use std::path::Path;
use crate::common::reserved_filename::LATEST_ROOT_ID_FILENAME;

fn write<W : Write>(write : &mut W, id : &RustIdentity) -> FsResult<()>{
    let mut vec : Vec<KVal> = vec![];
    vec.push(comp_int(id.random1() as i64));
    vec.push(comp_int(id.random2() as i64));
    dochy_compaction::encode(&vec, write)?;
    Ok(())
}

fn read<R : Read>(read : &mut R) -> FsResult<RustIdentity>{
    let (v, _) = dochy_compaction::decode(read)?;
    let mut i = v.iter();


    let r1 = i.next()?.as_i64()? as u64;
    let r2 = i.next()?.as_i64()? as u64;

    Ok(RustIdentity::create(r1, r2))
}

pub(crate) fn read_identity_file_data<P : AsRef<Path>>(hash_dir : P) -> FsResult<RustIdentity>{
    let mut file = std::fs::File::open(hash_dir.as_ref().join(LATEST_ROOT_ID_FILENAME))?;
    read(&mut file)
}

pub(crate) fn write_identity_file_data<P : AsRef<Path>>(hash_dir : P,
                                                        identity : &RustIdentity) -> FsResult<()>{
    let mut file = std::fs::File::create(hash_dir.as_ref().join(LATEST_ROOT_ID_FILENAME))?;
    write(&mut file, identity)
}


