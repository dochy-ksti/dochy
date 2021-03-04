use std::io::Read;
use crate::archiver::{ArcResult, Metadata};

/// Read only the metadata of the archive file and calculate the hash
/// ```
/// use dochy::archiver::{get_hash_and_metadata_from_dir, ArchiveOptions, ArcResult};
/// use std::fs::File;
/// use dochy::archiver::get_hash_and_metadata_from_archive;
///
/// fn main(){ fn2(); }
/// fn fn2() -> ArcResult<()>{
///     let mut archive_file = File::open("foo/some_file")?;
///     let (hash, metadata) = get_hash_and_metadata_from_archive(&mut archive_file)?;
///     Ok(())
/// }
/// ```
pub fn get_hash_and_metadata_from_archive(read : &mut impl Read) -> ArcResult<(u128, Metadata)> {
    let (meta, _size) = Metadata::from_bytes(read)?;
    let (_bytes, hash) = meta.to_bytes_and_hash()?;
    return Ok((hash, meta));
}