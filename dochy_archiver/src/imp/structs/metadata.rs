use std::time::{SystemTime, Duration};
use dochy_compaction::kval_enum::KVal;
use dochy_compaction::basic_compaction::{comp_str, comp_int};
use crate::imp::calc_hash::calc_hash;

use dochy_compaction::enc_dec::decode::decode;
use crate::ArcResult;
use crate::imp::structs::metadata_item::MetadataItem;
use crate::imp::structs::file_item::FileItem;
use std::io::Read;
use with_capacity_safe::vec_with_capacity_safe;
use std::path::Path;

#[derive(Debug, Clone, PartialEq)]
pub struct Metadata{
    vec : Vec<MetadataItem>
}

impl Metadata{
    pub(crate) fn new(vec : Vec<MetadataItem>) -> Self{ Self{ vec } }

    /// The metadata of each file.
    pub fn items(&self) -> &[MetadataItem]{ &self.vec }

    pub(crate) fn from_file_items(items : &[FileItem], root_path : &Path) -> ArcResult<Self>{
        let r : ArcResult<Vec<_>> = items.iter().map(|item| MetadataItem::from_file_item(item, root_path)).collect();
        Ok(Self::new(r?))
    }
    pub(crate) fn to_bytes(&self) -> ArcResult<Vec<u8>>{
        let mut r : Vec<KVal> = Vec::new();
        r.push(comp_int(self.vec.len() as i64));
        for item in &self.vec{
            r.push(comp_str(item.relative_path().to_string()));
            let d = item.modified().duration_since(SystemTime::UNIX_EPOCH)?;
            r.push(KVal::Int(d.as_millis() as i64));
            r.push(comp_int(item.len() as i64));
        }
        return Ok(dochy_compaction::enc_dec::encode_to_vec::encode_to_vec(&r)?);
    }


    pub(crate) fn from_bytes(bytes : &mut impl Read) -> ArcResult<(Metadata, usize)>{
        let (vec, bytes_read) = decode(bytes)?;
        let mut i = vec.iter();
        let len = i.next()?.as_i64()? as usize;
        let mut r : Vec<MetadataItem> = vec_with_capacity_safe(len)?;
        for _ in 0..len{
            let relative_path = i.next()?.as_string()?;
            let millis = i.next()?.as_i64()?;
            let duration = Duration::from_millis(millis as u64);
            let modified = SystemTime::UNIX_EPOCH.checked_add(duration)?;
            let bytes_len = i.next()?.as_i64()? as u64;
            r.push(MetadataItem::new(relative_path, modified, bytes_len))
        }

        Ok((Metadata::new(r), bytes_read))
    }


    pub(crate) fn to_bytes_and_hash(&self) -> ArcResult<(Vec<u8>, u128)>{
        let bytes = self.to_bytes()?;
        let hash = calc_hash(&bytes);
        Ok((bytes, hash))
    }

    ///calculate the hash of the metadata
    pub fn calc_hash(&self) -> ArcResult<u128>{
        self.to_bytes_and_hash().map(|(_,hash)| hash)
    }




}