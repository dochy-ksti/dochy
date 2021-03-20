use std::path::Path;
use crate::error::FsResult;
use std::fs::{File};
use crate::history::list_histories;

pub fn show_dir_contents_diff_history<P : AsRef<Path>>(proj_dir : P) -> FsResult<Vec<(u128, String, u64)>>{
    let proj_dir = proj_dir.as_ref();
    let histories = list_histories(proj_dir)?;
    let mut r : Vec<(u128, String, u64)> = vec![];
    for item in histories.list_files(){
        let name = item.props().calc_filename();
        let file = File::open(item.calc_path(proj_dir))?;
        r.push((item.hash(), name, file.metadata()?.len()))
    }
    Ok(r)
}