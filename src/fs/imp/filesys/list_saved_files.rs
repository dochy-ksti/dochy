use std::path::Path;
use crate::fs::imp::common::list::file_data::FileData;
use crate::fs::imp::common::list::list_files::list_files;
use crate::fs::error::FsResult;

pub fn list_saved_files<P : AsRef<Path>>(save_dir : P) -> FsResult<Vec<FileData>>{
    list_files(save_dir)
}