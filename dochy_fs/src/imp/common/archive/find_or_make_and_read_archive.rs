use std::path::{Path, PathBuf};
use crate::error::FsResult;
use dochy_archiver::{ArchiveData, create_archive_from_directory, read_archive_data};
use std::fs::File;
use std::io::Write;
use crate::imp::common::archive::archive_default_name::ARCHIVE_DEFAULT_NAME;
use crate::imp::common::path::created_time_file::{CREATED_TIME_FILE_NAME, create_time_dat};
use crate::imp::common::path::hash_dir_path::hash_dir_path;
use std::time::SystemTime;
use crate::imp::common::archive::archive_opt::JSON_ARC_OPT;

pub(crate) enum BufOrPath<'a>{
    SrcDir(&'a Path),
    Buf(&'a [u8]),
    ArchivePath(&'a Path),
}
pub(crate) fn make_archive_if_not_exist(proj_dir: &Path, hash : u128, bop : BufOrPath) -> FsResult<PathBuf>{
    let hash_dir = get_or_make_hash_dir_path(proj_dir, hash)?;
    let archive_path = hash_dir.join(ARCHIVE_DEFAULT_NAME);
    if archive_path.exists() == false{
        match bop{
            BufOrPath::Buf(bytes) =>{
                let mut file = File::create(&archive_path)?;
                file.write_all(bytes)?
            },
            BufOrPath::SrcDir(src_dir) =>{
                let mut file = File::create(&archive_path)?;

                create_archive_from_directory(src_dir, &mut file,
                                              |_| false, &*JSON_ARC_OPT)?;
            },
            BufOrPath::ArchivePath(path) =>{
                std::fs::copy(path, &archive_path)?;
            }
        }
    }
    Ok(archive_path)
}

pub(crate) fn find_or_make_and_read_archive(proj_dir: &Path, hash : u128, bop : BufOrPath) -> FsResult<ArchiveData>{
    let archive_path = make_archive_if_not_exist(proj_dir, hash, bop)?;
    let mut file = File::open(archive_path)?;
    return Ok(read_archive_data(&mut file)?);
}

fn get_or_make_hash_dir_path(proj_dir: &Path, hash : u128) -> FsResult<PathBuf>{
    let hash_dir_path = hash_dir_path(proj_dir, hash);
    if hash_dir_path.exists(){
        return Ok(hash_dir_path);
    }
    std::fs::create_dir(&hash_dir_path)?;
    let time_file_path = hash_dir_path.join(CREATED_TIME_FILE_NAME);
    let mut file = File::create(time_file_path)?;
    create_time_dat(SystemTime::now(), &mut file)?;
    Ok(hash_dir_path)
}
