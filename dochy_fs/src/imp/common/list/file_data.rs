use std::time::SystemTime;
use std::fs::DirEntry;
use crate::error::FsResult;
use std::path::{Path, PathBuf};
use crate::imp::common::path::hash_dir_path::hash_dir_path;

/// file's metadata
/// We presume we can get modified time of diff files under any circumstances.
/// list_saved_files doesn't work if this presumption is not true.
#[derive(Debug)]
pub struct FileData{
    hash : u128,
    modified : SystemTime,
    len : u64,
    name : String,
}

impl FileData{
    pub fn new(hash : u128, modified : SystemTime, name : String, len : u64) -> FileData{
        FileData{ hash, modified, name, len }
    }
    pub fn from(hash : u128, entry : &DirEntry) -> FsResult<FileData>{
        let metadata = entry.metadata()?;
        let modified = metadata.modified()?;
        let name = entry.file_name().to_string_lossy().to_string();
        let size = metadata.len();
        Ok(FileData::new(hash, modified, name, size))
    }

    pub fn hash(&self) -> u128{ self.hash }
    pub fn modified(&self) -> SystemTime{ self.modified }
    pub fn name(&self) -> &str{ &self.name }
    pub fn len(&self) -> u64{ self.len }

    pub fn calc_path<P : AsRef<Path>>(&self, proj_dir : P) -> PathBuf{
        let proj_dir = proj_dir.as_ref();
        let dir = hash_dir_path(proj_dir, self.hash);
        dir.join(self.name())
    }
}
