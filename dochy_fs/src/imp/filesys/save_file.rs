use std::path::{Path, PathBuf};
use std::fs::File;
use crate::error::FsResult;
use std::io::Write;
use dochy_core::structs::RootObject;
use crate::imp::common::current_src::CurrentSrc;
use crate::imp::common::archive::archive_src_dir::archive_src_dir;
use crate::imp::common::archive::archive_archive::archive_archive;
use crate::imp::common::path::hash_dir_path::hash_dir_path;

/// 常にsrc_dirを参照しながら、srcがアップデートされた場合、新しいディレクトリを作り、archiveファイルを用意し、
/// さらにセーブファイルも置く。セーブファイルは常にアーカイブと同じフォルダにある。
///
///
/// バージョン間のデータ変換をしっかり行う必要がある。archiveの状態から、srcを参照して最新のバージョンにアップデートする。
pub fn save_file<P : AsRef<Path>>(save_dir: P,
                                  root_obj : &RootObject,
                                  src : &CurrentSrc,
                                  file_name : &str,
                                  overwrite : bool) -> FsResult<PathBuf>{
    let save_dir = save_dir.as_ref();

    let (initial, hash) = match src {
        CurrentSrc::SrcDir(src_dir) => {
            archive_src_dir(src_dir, save_dir)?
        },
        CurrentSrc::ArchiveFile(archive_path) =>{
            archive_archive(archive_path, save_dir)?
        }
    };

    let file_path = hash_dir_path(save_dir, hash).join(file_name);
    if file_path.exists() && overwrite == false {
        Err("file already exists")?
    }

    let diff = dochy_diff::get_diff(&initial, &root_obj)?;
    let mut file = File::create(&file_path)?;
    file.write_all(&diff)?;
    Ok(file_path)
}



