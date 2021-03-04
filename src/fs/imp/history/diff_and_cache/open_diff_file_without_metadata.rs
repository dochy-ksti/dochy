use std::path::Path;
use crate::fs::error::FsResult;
use std::fs::File;

/// 最初のメタデータを省いて返す
pub(crate) fn open_diff_file_without_metadata(path : &Path) -> FsResult<File>{
    let mut file = File::open(&path)?;

    let (_first, _) = crate::compaction::decode(&mut file)?;

    Ok(file)
}