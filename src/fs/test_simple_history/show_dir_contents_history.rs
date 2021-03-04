use std::path::{Path};
use crate::fs::error::FsResult;
use std::fs::{read_dir};
use std::time::SystemTime;

pub fn show_dir_contents_history<P : AsRef<Path>>(path : P) -> FsResult<Vec<(String, usize)>>{
    let mut r : Vec<(SystemTime, String,usize)> = vec![];
    let dir = read_dir(path)?;

    for item in dir{
        let entry = item?;

        let name = entry.path().file_name()?.to_string_lossy().to_string();
        let size = entry.metadata()?.len() as usize;
        let time = entry.metadata()?.created()?;
        r.push((time, name,size));
    }
    r.sort_by_key(|a| a.0);
    let r : Vec<(String, usize)> = r.into_iter().map(|(_,s,u)| (s,u)).collect();
    Ok(r)
}
