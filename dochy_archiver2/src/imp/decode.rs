use crate::imp::structs::archive_data::ArchiveDataItem;
use crate::{ArcResult, ArchiveData};
use crate::imp::structs::archiver::Archiver;

pub(crate) fn decode<T : Send + 'static>(data : &[u8]) -> ArcResult<Vec<u8>> {
    let mut decoder = snap::raw::Decoder::new();
    Ok(decoder.decompress_vec(data).or_else(|e| Err(format!("{}", e)))?)

}