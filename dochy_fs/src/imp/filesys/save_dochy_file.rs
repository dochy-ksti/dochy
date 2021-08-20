use std::path::{Path, PathBuf};
use std::fs::File;
use crate::error::FsResult;
use std::io::Write;
use dochy_core::structs::RootObject;
use crate::imp::common::prepare_hash_dir::prepare_hash_dir;
use crate::common::CurrentSrc;
use crate::imp::filesys::save_dir_info::SaveDirInfo;

/// 常にsrc_dirを参照しながら、srcがアップデートされた場合、新しいディレクトリを作り、archiveファイルを用意し、
/// さらにセーブファイルも置く。セーブファイルは常にアーカイブと同じフォルダにある。
///
///
/// バージョン間のデータ変換をしっかり行う必要がある。archiveの状態から、srcを参照して最新のバージョンにアップデートする。
pub fn save_dochy_file<P : AsRef<Path>>(info : &SaveDirInfo,
                                        file_name : &str,
                                        root: &RootObject,
                                        overwrite : bool) -> FsResult<PathBuf>{
    let save_dir = info.save_dir();
    let hash_dir = prepare_hash_dir(save_dir, info.current_src(), info.hash())?;

    let file_path = hash_dir.join(file_name);
    if file_path.exists() && overwrite == false {
        Err("file already exists")?
    }
    let diff = dochy_diff::get_diff(&src_root, &root)?;
    let mut file = File::create(&file_path)?;
    file.write_all(&diff)?;
    Ok(file_path)
}



