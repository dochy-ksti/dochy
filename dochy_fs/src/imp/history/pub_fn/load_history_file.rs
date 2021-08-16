use crate::error::FsResult;
use dochy_core::structs::RootObject;
use crate::imp::history::fs::load::load;
use crate::history::{FileHistory, HistoryOptions, get_current_root_obj_info};
use crate::imp::history::file_name::file_name_props::FileNameProps;
use std::path::Path;
use crate::imp::common::path::hash_dir_path::hash_dir_path;
use dochy_core::{ adjust_versions};
use crate::imp::history::file_hist::history_file_data::HistoryFileData;
use crate::imp::history::current_root_obj_info::current_root_obj_info::{set_current_root_obj_info, CurrentRootObjInfo};
use crate::imp::common::dochy_cache::DochyCache;

/// Loads a history file.
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

    let guard = get_current_root_obj_info(history_dir, hash);
    match load_impl(history_dir, hash, props, history, cache, op, validation){
        Ok(root) =>{
            *guard = Some(CurrentRootObjInfo::new(root.id(), props.clone()));

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

    if cache.hash() != hash{
        let root = cache.get_or_create_hash_root(dir, hash)?;
        let loaded = load(&file_path, history, root, cache,  op)?;
        let adjusted = adjust_versions(cache.clone_src_root(), loaded, validation)?;
        Ok(adjusted)
    } else{
        let root = cache.clone_src_root();
        let loaded = load(&file_path, history, root, cache,op)?;
        Ok(loaded)
    }
}

pub fn load_history_file_data<P : AsRef<Path>>(history_dir : P,
                                               data : &HistoryFileData,
                                               cache : &mut DochyCache,
                                               op : &HistoryOptions,
                                               validation : bool) -> FsResult<RootObject> {
    load_history_file(history_dir, data.hash(), data.props(), data.history(), cache, op, validation)
}