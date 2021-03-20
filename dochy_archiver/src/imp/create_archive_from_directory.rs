use std::path::{Path};
use crate::{ArcResult, ArchiveOptions};
use std::io::{Write};
use std::fs::File;
use crate::imp::get_hash_and_metadata_from_dir::{get_hash_metadata_and_bytes};

pub enum CreateArchiveFromDirectory{
    Canceled(u128),
    WrittenSuccessfully(u64, u128)
}

/// Archive a directory to the archive file. ArchiveOptions customise how it's archived.
///
/// cancel_by_hash must return if the archiving process is canceled.
///
/// If the canceling isn't needed, return false.
///```
/// use dochy_archiver::{create_archive_from_directory, ArchiveOptions};
///
/// let mut archive : Vec<u8> = vec![];
/// let archive = create_archive_from_directory(
///     "foo/some_dir_path",
///     &mut archive,
///     |_hash| false,
///     &ArchiveOptions::new());
///```
/// The hash is calculated by every file's filename, modified time and file size.
///
/// If you record the hash and compare, you can cancel the archiving process when no file isn't modified.
///```
/// use dochy_archiver::ArcResult;
/// fn main(){
///     fn2();
/// }
/// fn fn2() -> ArcResult<()>{
/// use dochy::archiver::{create_archive_from_directory, ArchiveOptions, CreateArchiveFromDirectory};
/// use std::path::Path;
/// use std::fs::File;
/// use std::io::Write;
///     let mut buf : Vec<u8> = vec![];
///     let r = create_archive_from_directory("foo/some_dir_path", &mut buf,
///         |hash|{
///             //If the same hash already exists, all the files are not modified,
///             //so you can safely cancel the process
///             Path::new("bar/some_dir2").join(format!("{}", hash)).exists()
///         }, &ArchiveOptions::new())?;
///     if let CreateArchiveFromDirectory::WrittenSuccessfully(_size, hash) = r{
///         //record the hash and the archived data in some way.
///         let mut file = File::create(Path::new("bar/some_dir2").join(format!("{}", hash)))?;
///         file.write_all(&buf);
///     }
///     Ok(())
/// }
///```
pub fn create_archive_from_directory<P : AsRef<Path>>(dir_path: P,
                                                      write : &mut impl Write,
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
