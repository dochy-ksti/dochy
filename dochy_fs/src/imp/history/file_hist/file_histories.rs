use crate::imp::history::file_hist::file_history::FileHistory;
use crate::imp::history::file_hist::history_file_data::HistoryFileData;
use std::path::Path;
use crate::imp::common::path::hash_dir_path::hash_dir_path;
use std::ops::Range;
use crate::common::remove_hash_dir;
use crate::error::FsResult;

/// Represents every file history in every hash directory of a project
pub struct FileHistories{
    vec : Vec<(u128, FileHistory)>,
}

impl FileHistories{
    pub(crate) fn new(vec : Vec<(u128, FileHistory)>) -> FileHistories{ FileHistories{ vec } }

    /// list every HistoryFileData chronologically
    pub fn list_files(&self) -> Vec<HistoryFileData>{
        self.vec.iter().flat_map(|(hash, his)|{
            //let hash = *hash;
            his.list_files().into_iter()
                .map(move |props| HistoryFileData::new(*hash, his, props))
        }).collect()
    }

    /// gets the newest HisotryFileData
    pub fn get_newest_file_data(&self) -> Option<HistoryFileData>{
        self.vec.last().and_then(|(hash, his)|
                                     his.get_newest_prop().map(|prop|
                                     HistoryFileData::new(*hash, his, prop)))
    }

    /// remove old files other than latest n files. This function consumes history
    pub fn remove_old_files<P : AsRef<Path>>(self, keep_latest : usize, history_dir : P) -> FsResult<()>{
        let mut s = self;
        unsafe{ s.remove_old_files_us(keep_latest, history_dir) }
    }

    /// remove old files other than latest n files. This function didn't update the history data,
    /// so the data will be inconsistent with actual files
    pub unsafe fn remove_old_files_us<P : AsRef<Path>>(&mut self, keep_latest : usize, history_dir : P) -> FsResult<()>{
        if self.vec.len() == 0{ return Ok(()); }
        let (hash, his) = self.vec.last().unwrap();
        let hash_dir_path = hash_dir_path(history_dir.as_ref(), *hash);
        if keep_latest < his.list_files().len(){
            let len = self.vec.len();
            if 2 <= len {
                remove_hash_dirs(&mut self.vec, 0..len - 1, history_dir.as_ref())?;
            }
            let (_,last) = self.vec.remove(0);
            last.remove_old_files(keep_latest, hash_dir_path);
            return Ok(());
        }

        let mut sum = 0;
        let mut ind = self.vec.len() - 1;

        //hash_dir単位でのいい加減な削除。
        while 1 <= ind{
            let index = ind;
            ind -= 1;

            let (_, his) = self.vec.get(index).unwrap();
            let len = his.list_files().len();
            sum += len;
            if keep_latest < sum{

                remove_hash_dirs(&mut self.vec,0..index, history_dir.as_ref())?;
                return Ok(());
            }
        }
        Ok(())
    }
}

fn remove_hash_dirs(vec : &mut Vec<(u128, FileHistory)>, range : Range<usize>, history_dir : &Path) -> FsResult<()>{
    for (hash, _) in vec.drain(range){
        remove_hash_dir(history_dir, hash)?;
    }
    Ok(())
}