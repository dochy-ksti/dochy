use std::path::{Path};
use crate::error::FsResult;
use crate::imp::history::file_hist::file_history::FileHistory;
use crate::imp::history::file_name::analyze_file_name::analyze_file_name;


pub(crate) fn create_file_history(history_hash_dir: &Path, hint_max_phase : Option<usize>) -> FsResult<FileHistory>{
    let dir = std::fs::read_dir(history_hash_dir)?;
    let mut history = FileHistory::new();
    for entry in dir {
        let entry = entry?;
        let filename = entry.path().file_name()?.to_string_lossy().to_string();
        //dbg!(format!("filename {}", &filename));
        if let Some(props) = analyze_file_name(&filename, hint_max_phase) {
            history.add(props)
        }
    }
    Ok(history)
}