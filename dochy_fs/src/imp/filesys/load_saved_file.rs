use std::path::Path;
use crate::error::FsResult;
use dochy_core::structs::RootObject;
use std::fs::File;
use dochy_diff::apply_diff;
use dochy_core::{adjust_versions};
use crate::imp::common::archive::load_archive::load_archive;
use crate::imp::common::path::reserved_filename::ARCHIVE_DEFAULT_NAME;
use crate::common::DochyCache;
use crate::common::hash::folder_name_to_hash;

/// archiveファイルは常にファイルと同じディレクトリにあることになっている。
pub fn load_saved_file<P : AsRef<Path>>(file_path : P, cache : &DochyCache, validation : bool) -> FsResult<RootObject>{
    let path = file_path.as_ref();
    let dir_path = path.parent().ok_or("file_path's file must be in a folder which contains src.archive file.")?;
    let hash = folder_name_to_hash(dir_path.file_name()?)?;

    let mut root = if cache.hash() == hash{
        cache.clone_src_root()
    } else {
        //archiveに関しては、カレントしかキャッシュしないことにする
        let archive_path = dir_path.join(ARCHIVE_DEFAULT_NAME);
        load_archive(archive_path, validation)?
    };

    let mut file = File::open(file_path)?;
    apply_diff(&mut root, &mut file)?;

    if cache.hash() != hash{
        Ok(adjust_versions(cache.clone_src_root(), root, validation)?)
    } else{
        Ok(root)
    }
}