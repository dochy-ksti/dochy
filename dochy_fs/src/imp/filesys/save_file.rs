use std::path::{Path, PathBuf};
use std::fs::File;
use crate::error::FsResult;
use std::io::Write;
use dochy_core::structs::RootObject;
use crate::imp::common::dochy_cache::DochyCache;
use crate::imp::common::prepare_hash_dir::prepare_hash_dir;

/// 常にsrc_dirを参照しながら、srcがアップデートされた場合、新しいディレクトリを作り、archiveファイルを用意し、
/// さらにセーブファイルも置く。セーブファイルは常にアーカイブと同じフォルダにある。
///
///
/// バージョン間のデータ変換をしっかり行う必要がある。archiveの状態から、srcを参照して最新のバージョンにアップデートする。
pub fn save_file<P : AsRef<Path>>(save_dir: P,
                                  root_obj : &RootObject,
                                  cache : &DochyCache,
                                  file_name : &str,
                                  overwrite : bool) -> FsResult<PathBuf>{
    let save_dir = save_dir.as_ref();
    let hash_dir = prepare_hash_dir(save_dir, cache.current_src(), cache.hash())?;

    let file_path = hash_dir.join(file_name);
    if file_path.exists() && overwrite == false {
        Err("file already exists")?
    }
    let src_root = cache.clone_src_root();
    let diff = dochy_diff::get_diff(&src_root, &root_obj)?;
    let mut file = File::create(&file_path)?;
    file.write_all(&diff)?;
    Ok(file_path)
}



