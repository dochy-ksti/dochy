use crate::fs::error::FsResult;
use crate::core::structs::RootObject;
use crate::fs::imp::history::fs::load::load;
use crate::fs::imp::history::diff_and_cache::dochy_cache::DochyCache;
use crate::fs::history::{FileHistory, HistoryOptions};
use crate::fs::imp::history::file_name::file_name_props::FileNameProps;
use std::path::Path;
use crate::fs::imp::common::path::hash_dir_path::hash_dir_path;
use crate::fs::imp::common::current_src::CurrentSrc;
use crate::core::{json_dir_to_rust, adjust_versions};
use crate::fs::imp::common::archive::load_archive::load_archive;
use crate::fs::imp::history::file_hist::history_file_data::HistoryFileData;

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
            let new = json_dir_to_rust(src_dir, validation)?;
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