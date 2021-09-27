use std::io::Read;
use crate::imp::structs::metadata_b::Metadata;
use crate::ArcResult;
use std::collections::{BTreeMap};
use crate::imp::structs::archive_data_b::ArchiveData;
use with_capacity_safe::vec_with_capacity_safe;


pub fn read_archive_data(read : &mut impl Read) -> ArcResult<ArchiveData>{
    let (meta, _size) = Metadata::from_bytes(read)?;
    let mut map : BTreeMap<String, Vec<u8>> = BTreeMap::new();

    let mut decoder = snap::read::FrameDecoder::new(read);

    for item in meta.items(){
        let mut buf = vec_with_capacity_safe(item.len() as usize)?;
        unsafe{ buf.set_len(item.len() as usize); }

        decoder.read_exact(&mut buf)?;
        map.insert(item.relative_path().to_string(), buf);
    }

    Ok(ArchiveData::new(meta, map))
}