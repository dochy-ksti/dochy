use std::path::Path;
use crate::imp::structs::metadata::Metadata;
use crate::imp::get_file_items_from_directory::get_file_items_from_directory;
use crate::{ArcResult, ArchiveOptions};



pub(crate) fn get_hash_metadata_and_bytes(dir_path: &Path, opt : &ArchiveOptions) -> ArcResult<(u128, Metadata, Vec<u8>)> {
    let file_items = get_file_items_from_directory(dir_path, opt)?;
    let meta = Metadata::from_file_items(&file_items, dir_path)?;
    let (bytes, hash) = meta.to_bytes_and_hash()?;
    Ok((hash, meta, bytes))
}



pub fn get_hash_and_metadata_from_dir<P : AsRef<Path>>(dir_path : P, opt : &ArchiveOptions) -> ArcResult<(u128, Metadata)>{
    let (h, m, _) = get_hash_metadata_and_bytes(dir_path.as_ref(), opt)?;
    Ok((h,m))
}