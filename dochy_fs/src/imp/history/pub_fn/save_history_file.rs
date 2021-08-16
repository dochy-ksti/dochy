use crate::error::FsResult;
use std::path::Path;
use crate::history::HistoryOptions;
use dochy_core::structs::RootObject;
use crate::imp::history::fs::start_new::start_new as fs_start_new;
use crate::imp::history::current_root_obj_info::current_root_obj_info::{get_current_root_obj_info, CurrentRootObjInfo};
use crate::imp::history::file_name::file_name_props::FileNameProps;
use crate::imp::history::file_hist::create_file_history::create_file_history;
use crate::imp::history::fs::derive_impl::derive_impl;
use crate::imp::common::dochy_cache::DochyCache;
use crate::imp::common::prepare_hash_dir::prepare_hash_dir;
use std::sync::Weak;

/// calculates the diff from the latest save file(most of the time) and save the diff file.
///
/// # Arguments
///
/// * `history_dir` - the directory which has hash directories which contains history files.
/// * `tag` - arbitrary string to distinguish files. It's appended to the file name.
/// * 'root' - the object to save
/// * 'cache' - Cached data to make the process faster.
///
/// The algorithm to generate diffs is described [here](https://github.com/dochy-ksti/dochy/blob/master/dochy_manual/src/sample_test/sample_code/history.md)
pub fn save_history_file<P : AsRef<Path>, Op : AsRef<HistoryOptions>>(history_dir: P,
                             tag : Option<String>,
                             root : &RootObject,
                             cache : &mut DochyCache, opt : Op) -> FsResult<FileNameProps> {
    save_history_file_impl(history_dir, tag, root, root.id(), cache, opt)
}
pub struct JoinHandler<T>{
    handle : std::thread::JoinHandle<T>
}

impl<T> JoinHandler<T>{
    pub fn join(self) -> FsResult<T>{
        Ok(self.handle.join().or(Err("join failed"))?)
    }
}

pub fn save_history_file_async<
    P : AsRef<Path>,
    Op : AsRef<HistoryOptions>,
    F : FnOnce(FsResult<FileNameProps>) + Send>(history_dir: P,
                                         tag : Option<String>,
                                         root : &RootObject,
                                         cache : &mut DochyCache,
                                         opt : Op,
                                         callback : F) -> JoinHandler<Option<FileNameProps>> {
    let id = root.id();
    let clone = root.clone();
    let history_dir = history_dir.as_ref().to_path_buf();
    let opt = opt.as_ref().clone();
    let handle = std::thread::spawn(move || {
        let result = save_history_file_impl(history_dir, tag, &clone, id, cache, opt);
        match result {
            Ok(r) => {
                let ret = r.clone();
                callback(Ok(r));
                return Some(ret);
            },
            Err(e) =>{
                callback(Err(e));
                return None;
            }
        }
    });
    JoinHandler{ handle }
}

fn save_history_file_impl<P : AsRef<Path>, Op : AsRef<HistoryOptions>>(history_dir: P,
                                                                      tag : Option<String>,
                                                                      root : &RootObject,
                                                                      root_id : Weak<()>,
                                                                      cache : &mut DochyCache, opt : Op) -> FsResult<FileNameProps> {

    let history_dir = history_dir.as_ref();
    let opt = opt.as_ref();
    let src = cache.current_src();
    let hash = cache.hash();
    let history_hash_dir = prepare_hash_dir(history_dir, src, hash)?;

    //TODO: ただしくMutexを管理し、マルチスレッドでも同期化すること
    let mut guard = get_current_root_obj_info(history_dir, hash);
    let info = guard.as_ref().map(|a| a.clone());

    if let Some(info) = &info {
        if root_id.ptr_eq(info.current_root_id()) {
            let history = create_file_history(&history_hash_dir, opt.max_phase(), opt.is_cumulative())?;
            if let Some(newest) = history.get_newest_prop() {
                let from = if info.current_base_file().phase() == opt.max_phase() && info.current_base_file() != newest {
                    //最新ファイルからの派生でない場合、キャッシュに乗っていない可能性がより高いし、最終フェーズの計算が面倒でもあるので、親から派生する
                    history.get_parent(info.current_base_file())?
                } else {
                    info.current_base_file()
                };

                let latest = derive_impl(tag, root, cache, &history_hash_dir, &history, from, opt)?;
                *guard = Some(CurrentRootObjInfo::new(root.id(), latest.clone()));

                return Ok(latest);
            }
        }
    }

    let opt = opt.as_ref();
    let latest = fs_start_new(tag, root, cache, &history_hash_dir, opt, opt.is_cumulative())?;
    *guard = Some(CurrentRootObjInfo::new(root.id(), latest.clone()));
    return Ok(latest);
}
