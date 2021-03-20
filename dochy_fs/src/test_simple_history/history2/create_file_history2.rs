use std::path::{Path};
use crate::error::FsResult;
use crate::imp::history::file_name::analyze_file_name::analyze_file_name;
use crate::test_simple_history::history2::file_history2::FileHistory2;
use dochy_compaction::enc_dec::decode::decode;


pub(crate) fn create_file_history2(dir_path : &Path, hint_max_phase : Option<usize>) -> FsResult<FileHistory2>{
    let dir = std::fs::read_dir(dir_path)?;
    let mut history = FileHistory2::new();
    for entry in dir {
        let entry = entry?;
        let filename = entry.path().file_name()?.to_string_lossy().to_string();
        let len = entry.metadata()?.len();

        //dbg!(format!("filename {}", &filename));
        if let Some(props) = analyze_file_name(&filename, hint_max_phase) {
            let mut file = std::fs::File::open(entry.path())?;
            let (_,tag_size) = decode(&mut file)?;
            //dbg!(tag_size);
            let size = len - tag_size as u64;
            history.add(props, size)
        }
    }
    Ok(history)
}