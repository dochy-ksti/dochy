use crate::error::FsResult;
use std::path::Path;
use crate::history::{HistoryOptions, CurrentRootObjInfo};
use dochy_core::structs::RootObject;
use crate::imp::history::fs::start_new::start_new as fs_start_new;
use crate::imp::history::file_name::file_name_props::FileNameProps;
use crate::imp::history::file_hist::create_file_history::create_file_history;
use crate::imp::history::fs::derive_impl::derive_impl;
use crate::imp::common::dochy_cache::DochyCache;
use crate::imp::common::prepare_hash_dir::prepare_hash_dir;
use std::sync::Weak;
use crate::imp::history::history_info::HistoryInfo;
use crate::imp::history::current_root_obj_info::history_cache_map::get_mutex;

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
pub fn save_history_file(history_info : &HistoryInfo,
                         tag : Option<String>,
                         root : &RootObject) -> FsResult<FileNameProps> {
    save_history_file_impl(history_info, tag, root, root.id())
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
    F : FnOnce(FsResult<FileNameProps>) + Send + 'static>(history_info: &HistoryInfo,
                                                tag : Option<String>,
                                                root : &RootObject,
                                                callback : F) -> JoinHandler<Option<FileNameProps>> {
    let id = root.id();
    let clone = root.clone();
    let history_info = history_info.clone();
    let handle = std::thread::spawn(move || {
        let result = save_history_file_impl(&history_info, tag, &clone, id);
        match result {
            Ok(r) => {
                let ret = r.clone();
                callback(Ok(r));
                Some(ret)
            },
            Err(e) => {
                callback(Err(e));
                None
            }
        }
    });
    JoinHandler{ handle }
}

fn save_history_file_impl(history_info: &HistoryInfo,
                          tag : Option<String>,
                          root : &RootObject,
                          root_id : Weak<()>) -> FsResult<FileNameProps> {
    let history_dir = history_info.history_dir();
    let mut mutex = get_mutex(history_dir)?;
    let info = mutex.current_info().clone();

    let opt = info.history_options();
    let src = info.current_src();
    let (cache, h) = mutex.muts();
    let hash = cache.hash();
    let history_hash_dir = prepare_hash_dir(history_dir, src, hash)?;

    if let Some(info) = h {
        if root_id.ptr_eq(info.current_root_id()) {
            let history = create_file_history(&history_hash_dir, opt.max_phase(), opt.is_cumulative())?;
            if let Some(newest) = history.get_newest_prop() {
                let from = if info.current_base_file().phase() == opt.max_phase() && info.current_base_file() != newest {
                    //最新ファイルからの派生でない場合、キャッシュに乗っていない可能性がより高いし、最終フェーズの計算が面倒でもあるので、親から派生する
                    history.get_parent(info.current_base_file())?
                } else {
                    info.current_base_file()
                };

                let saved = derive_impl(tag, root, cache, &history_hash_dir, &history, from, opt)?;
                *h = Some(CurrentRootObjInfo::new(root_id, saved.clone()));

                return Ok(saved);
            }
        }
    }

    let saved = fs_start_new(tag, root, cache, &history_hash_dir, opt, opt.is_cumulative())?;
    *h = Some(CurrentRootObjInfo::new(root_id, saved.clone()));
    return Ok(saved);
}
