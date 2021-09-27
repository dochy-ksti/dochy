// use std::io::Read;
// use crate::{ArcResult, Metadata};
//
//
// pub fn get_hash_and_metadata_from_archive(read : &mut impl Read) -> ArcResult<(u128, Metadata)> {
//     let (meta, _size) = Metadata::from_bytes(read)?;
//     let (_bytes, hash) = meta.to_bytes_and_hash()?;
//     return Ok((hash, meta));
// }