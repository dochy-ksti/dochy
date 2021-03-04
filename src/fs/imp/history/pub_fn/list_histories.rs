use std::path::Path;
use crate::fs::history::FileHistory;
use crate::fs::common::get_hash_times;
use crate::fs::error::FsResult;
use crate::fs::imp::history::file_hist::file_histories::FileHistories;

pub fn list_histories<P:AsRef<Path>>(history_dir : P) -> FsResult<FileHistories>{
    let history_dir = history_dir.as_ref();
    let hash_times = get_hash_times(history_dir)?;

    let mut vec : Vec<(u128, FileHistory)> = vec![];
    for (hash, _time) in hash_times{
        if let Ok(history) = FileHistory::create(history_dir, hash){
            vec.push((hash, history));
        }
    }

    Ok(FileHistories::new(vec))
}