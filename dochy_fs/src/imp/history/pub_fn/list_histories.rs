use std::path::Path;
use crate::history::FileHistory;
use crate::common::get_hash_times;
use crate::error::FsResult;
use crate::imp::history::file_hist::file_histories::FileHistories;


pub fn list_histories<P:AsRef<Path>>(history_dir : P, max_phase : usize, cumulative : bool) -> FsResult<FileHistories>{
    let history_dir = history_dir.as_ref();
    let hash_times = get_hash_times(history_dir)?;

    let mut vec : Vec<(u128, FileHistory)> = vec![];
    for (hash, _time) in hash_times{
        if let Ok(history) = FileHistory::create(history_dir, hash, max_phase, cumulative){
            vec.push((hash, history));
        }
    }

    Ok(FileHistories::new(vec))
}