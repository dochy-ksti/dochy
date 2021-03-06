use crate::error::FsResult;
use dochy_core::structs::RootObject;
use crate::imp::history::fs::load::load;
use crate::imp::history::diff_and_cache::dochy_cache::DochyCache;
use crate::history::{FileHistory, HistoryOptions};
use crate::imp::history::file_name::file_name_props::FileNameProps;
use std::path::Path;
use crate::imp::common::path::hash_dir_path::hash_dir_path;
use crate::imp::common::current_src::CurrentSrc;
use dochy_core::{json_dir_to_root, adjust_versions};
use crate::imp::common::archive::load_archive::load_archive;
use crate::imp::history::file_hist::history_file_data::HistoryFileData;

pub fn load_history_file<P : AsRef<Path>>(history_dir : P,
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

    match cache.current_src() {
        CurrentSrc::SrcDir(src_dir) => {
            let new = json_dir_to_root(src_dir, validation)?;
            let adjusted = adjust_versions(new, loaded, validation)?;
            Ok(adjusted)
        },
        CurrentSrc::ArchiveFile(current_archive) => {
            let new = load_archive(current_archive, validation)?;
            let adjusted = adjust_versions(new, loaded, validation)?;
            Ok(adjusted)
        }
    }
}

pub fn load_history_file_data<P : AsRef<Path>>(history_dir : P,
                                               data : &HistoryFileData,
                                               cache : &mut DochyCache,
                                               op : &HistoryOptions,
                                               validation : bool) -> FsResult<RootObject> {
    load_history_file(history_dir, data.hash(), data.props(), data.history(), cache, op, validation)
}