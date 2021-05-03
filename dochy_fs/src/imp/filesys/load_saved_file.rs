use std::path::Path;
use crate::error::FsResult;
use dochy_core::structs::RootObject;
use std::fs::File;
use dochy_diff::apply_diff;
use dochy_core::{adjust_versions, json_dir_to_root};
use crate::imp::common::current_src::CurrentSrc;
use crate::imp::common::archive::load_archive::load_archive;
use crate::imp::common::path::reserved_filename::ARCHIVE_DEFAULT_NAME;

/// archiveファイルは常にファイルと同じディレクトリにあることになっている。
pub fn load_saved_file<P : AsRef<Path>>(file_path : P, current_src : &CurrentSrc, validation : bool) -> FsResult<RootObject>{
    let path = file_path.as_ref();
    let dir_path = path.parent().ok_or("file_path's file must be in a folder which contains src.archive file.")?;
    let archive_path = dir_path.join(ARCHIVE_DEFAULT_NAME);
    load_file_separate(path, archive_path, current_src, validation)
}

fn load_file_separate<P1 : AsRef<Path>, P2 : AsRef<Path>>(
    file_path : P1,
    archive_path : P2,
    current_src : &CurrentSrc,
    validation : bool) -> FsResult<RootObject>{

    let file_path = file_path.as_ref();
    let archive_path = archive_path.as_ref();

    let mut root = load_archive(archive_path, validation)?;

    let mut file = File::open(file_path)?;
    apply_diff(&mut root, &mut file)?;

    match current_src{
        CurrentSrc::SrcDir(src_dir) =>{
            let new = json_dir_to_root(src_dir, validation)?;
            let adjusted = adjust_versions(new, root, validation)?;
            Ok(adjusted)
        },
        CurrentSrc::ArchiveFile(current_archive) =>{
            let new = load_archive(current_archive, validation)?;
            let adjusted = adjust_versions(new, root, validation)?;
            Ok(adjusted)
        }
    }
}