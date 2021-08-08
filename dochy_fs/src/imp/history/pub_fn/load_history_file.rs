use crate::error::FsResult;
use dochy_core::structs::RootObject;
use crate::imp::history::fs::load::load;
use crate::history::{FileHistory, HistoryOptions};
use crate::imp::history::file_name::file_name_props::FileNameProps;
use std::path::Path;
use crate::imp::common::path::hash_dir_path::hash_dir_path;
use dochy_core::{ adjust_versions};
use crate::imp::history::file_hist::history_file_data::HistoryFileData;
use crate::imp::history::latest_file_info::latest_file_info::{set_current_root_obj_info, CurrentRootObjInfo};
use crate::imp::common::dochy_cache::DochyCache;

/// Loads a history file.
/// Concurrent access to a history_dir is not supported.
/// Concurrent load and save, save and save, load and load access can cause problems,
/// so use synchronization if you want to access concurrently.
pub fn load_history_file<
    P : AsRef<Path>,
    Op : AsRef<HistoryOptions>>(history_dir : P,
                                hash : u128,
                                props : &FileNameProps,
                                history : &FileHistory,
                                cache : &mut DochyCache,
                                op : Op,
                                validation : bool) -> FsResult<RootObject> {
    let history_dir = history_dir.as_ref();
    let op = op.as_ref();

    match load_impl(history_dir, hash, props, history, cache, op, validation){
        Ok(root) =>{
            let is_latest = history.get_newest_prop()? == props;
            set_current_root_obj_info(history_dir, hash,
                                      Some(CurrentRootObjInfo::new(root.id(), props.clone(), is_latest)));

            Ok(root)
        },
        Err(e) => Err(e),
    }
}

fn load_impl<P : AsRef<Path>>(history_dir : P,
                              hash : u128,
                              props : &FileNameProps,
                              history : &FileHistory,
                              cache : &mut DochyCache,
                              op : &HistoryOptions,
                              validation : bool) -> FsResult<RootObject> {
    let dir = history_dir.as_ref();
    let hash_dir = hash_dir_path(dir, hash);
    let file_path = hash_dir.join(props.calc_filename());
    let loaded = load(&file_path, history, cache, op)?;

    let src_root = cache.clone_src_root();
    let adjusted = adjust_versions(src_root, loaded, validation)?;
    Ok(adjusted)
}

pub fn load_history_file_data<P : AsRef<Path>>(history_dir : P,
                                               data : &HistoryFileData,
                                               cache : &mut DochyCache,
                                               op : &HistoryOptions,
                                               validation : bool) -> FsResult<RootObject> {
    load_history_file(history_dir, data.hash(), data.props(), data.history(), cache, op, validation)
}