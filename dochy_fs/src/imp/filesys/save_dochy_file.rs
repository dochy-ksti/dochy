use std::path::{PathBuf};
use std::fs::File;
use crate::error::FsResult;
use std::io::Write;
use dochy_core::structs::RootObject;
use crate::imp::filesys::save_dir_info::SaveDirInfo;
use crate::imp::filesys::save_cache_map::{get_mutex};
use crate::imp::common::path::prepare_hash_dir::prepare_hash_dir;
use crate::common::JoinHandler;

/// 常にsrc_dirを参照しながら、srcがアップデートされた場合、新しいディレクトリを作り、archiveファイルを用意し、
/// さらにセーブファイルも置く。セーブファイルは常にアーカイブと同じフォルダにある。
///
///
/// バージョン間のデータ変換をしっかり行う必要がある。archiveの状態から、srcを参照して最新のバージョンにアップデートする。
pub fn save_dochy_file(info : &SaveDirInfo,
                       file_name : &str,
                       root: &RootObject,
                       overwrite : bool) -> FsResult<PathBuf>{
    let mutex = get_mutex(info.save_dir())?;

    let info = mutex.cache();
    let save_dir = info.save_dir();
    let hash_dir = prepare_hash_dir(save_dir, info.current_src(), info.hash())?;
    let src_root = info.clone_src_root();

    let file_path = hash_dir.join(file_name);
    if file_path.exists() && overwrite == false {
        Err("file already exists")?
    }
    let diff = dochy_diff::get_diff(&src_root, &root)?;
    let mut file = File::create(&file_path)?;
    file.write_all(&diff)?;
    Ok(file_path)
}

pub fn save_dochy_file_async<
    F : FnOnce(FsResult<PathBuf>) + Send + 'static>(info : &SaveDirInfo,
                                                    file_name : &str,
                                                    root: &RootObject,
                                                    overwrite : bool,
                                                    callback : F) -> JoinHandler<Option<PathBuf>> {
    let info = info.clone();
    let file_name = file_name.to_string();
    let root = root.clone();
    let handle = std::thread::spawn(move ||{
        match save_dochy_file(&info, &file_name, &root, overwrite){
            Ok(p) =>{
                callback(Ok(p.to_path_buf()));
                Some(p)
            },
            Err(e) =>{
                callback(Err(e));
                None
            }
        }
    });
    JoinHandler::new(handle)
}

