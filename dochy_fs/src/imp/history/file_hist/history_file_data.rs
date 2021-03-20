use crate::imp::history::file_name::file_name_props::FileNameProps;
use std::path::{Path, PathBuf};
use crate::imp::common::path::hash_dir_path::hash_dir_path;
use crate::history::FileHistory;

#[derive(Debug, Clone, Copy)]
pub struct HistoryFileData<'a>{
    hash : u128,
    history : &'a FileHistory,
    props : &'a FileNameProps,
}

impl<'a> HistoryFileData<'a>{
    pub fn new(hash : u128, history : &'a FileHistory, props : &'a FileNameProps) -> HistoryFileData<'a>{ HistoryFileData{ hash, history, props } }

    pub fn hash(&self) -> u128{ self.hash }
    pub fn history(&self) -> &'a FileHistory{ &self.history }
    pub fn props(&self) -> &'a FileNameProps{ &self.props }

    pub fn calc_path<P : AsRef<Path>>(&self, history_dir : P) -> PathBuf{
        let hash_dir = hash_dir_path(history_dir.as_ref(), self.hash);
        hash_dir.join(self.props.calc_filename())
    }
}