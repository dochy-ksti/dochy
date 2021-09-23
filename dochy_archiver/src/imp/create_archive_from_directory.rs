use std::path::{Path};
use crate::{ArcResult, ArchiveOptions};
use std::io::{Write};
use std::fs::File;
use crate::imp::get_hash_and_metadata_from_dir::{get_hash_metadata_and_bytes};


pub fn create_archive_from_directory<P : AsRef<Path>, W : Write>(dir_path: P,
                                                      write : &mut W,
                                                      cancel_by_hash : impl Fn(u128)->bool,
                                                      opt : &ArchiveOptions) -> ArcResult<CreateArchiveFromDirectory>{
    let mut written_bytes : u64 = 0;

    let dir_path = dir_path.as_ref();

    let (hash, meta, bytes) = get_hash_metadata_and_bytes(dir_path, opt)?;
    if cancel_by_hash(hash){
        return Ok(CreateArchiveFromDirectory::Canceled(hash));
    }
    written_bytes += bytes.len() as u64;
    write.write_all(&bytes)?;

    let mut encoder = snap::write::FrameEncoder::new(write);

    for item in meta.items(){
        let path = item.calc_full_path(dir_path);
        let mut file = File::open(&path)?;
        let bytes = std::io::copy(&mut file, &mut encoder)?;
        written_bytes += bytes;
    }
    //encoderがout of scopeになって自動的にflushされる(はず
    Ok(CreateArchiveFromDirectory::WrittenSuccessfully(written_bytes, hash))
}
