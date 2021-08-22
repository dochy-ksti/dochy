use std::path::Path;
use crate::error::FsResult;
use dochy_core::structs::RootObject;
use std::fs::File;
use dochy_diff::apply_diff;
use dochy_core::{adjust_versions};
use crate::imp::common::archive::load_archive::load_archive;
use crate::imp::common::path::reserved_filename::ARCHIVE_DEFAULT_NAME;
use crate::common::hash::folder_name_to_hash;
use crate::imp::filesys::save_dir_info::SaveDirInfo;

pub fn load_dochy_file<P : AsRef<Path>>(file_path : P,
                                        current_save_dir_info : &SaveDirInfo,
                                        validation : bool) -> FsResult<RootObject>{
    let path = file_path.as_ref();
    let dir_path = path.parent().ok_or("file_path's file must be in a folder which contains src.archive file.")?;
    let hash = folder_name_to_hash(dir_path.file_name()?)?;

    let current_src_hash = current_save_dir_info.hash();
    let src_root = current_save_dir_info.clone_src_root();

    let mut root = if current_src_hash == hash{
        src_root.clone()
    } else {
        //archiveに関しては、カレントしかキャッシュしないことにする
        let archive_path = dir_path.join(ARCHIVE_DEFAULT_NAME);
        load_archive(archive_path, validation)?
    };

    let mut file = File::open(file_path)?;
    apply_diff(&mut root, &mut file)?;

    if current_src_hash != hash{
        Ok(adjust_versions(src_root, root, validation)?)
    } else{
        Ok(root)
    }
}