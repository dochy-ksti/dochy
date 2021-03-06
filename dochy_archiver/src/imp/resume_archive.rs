use std::path::Path;
use crate::{ArcResult, ArchiveData};
use std::collections::HashMap;
use std::fs::{create_dir, File, OpenOptions};
use std::io::Write;

/// Resume all the archived files to the directory
///
/// if overwrite == false and there's a file in the target path, or failed for other reason, Error is returned.
/// ```
/// use dochy_archiver::read_archive_data;
/// use std::fs::File;
/// use dochy_archiver::ArcResult;
/// use dochy_archiver::resume_archive;
///
/// fn main(){
///     fn2();
/// }
/// fn fn2() -> ArcResult<()>{
///     let mut archive_file = File::open("foo/some_file")?;
///     let archive_data = read_archive_data(&mut archive_file)?;
///     resume_archive("bar/target_dir", &archive_data, true)?;
///     Ok(())
/// }
/// ```
pub fn resume_archive<P : AsRef<Path>>(dir_path : P, data : &ArchiveData, overwrite : bool) -> ArcResult<()>{
    let dir_path = dir_path.as_ref();
    let mut folders = FolderItem::new();
    for (rel_path, _data) in data.iter() {
        let path = Path::new(rel_path);
        let ancs = get_ancestor_names(path)?;
        folders.set_ancestor_names(&ancs)
    }
    folders.create_folders(dir_path);

    for (rel_path, data) in data.iter(){
        let path = dir_path.join(rel_path);
        let mut file = if overwrite{
            File::create(&path)?
        } else {
            OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&path)?
        };

        file.write_all(data)?;
    }
    Ok(())
}

fn get_ancestor_names(path : &Path) -> ArcResult<Vec<String>>{
    let mut v : Vec<String> = vec![];
    for p in path.ancestors().skip(1){
        if let Some(filename) = p.file_name() {
            v.push(filename.to_string_lossy().to_string());
        }
    }
    v.reverse();
    Ok(v)
}

struct FolderItem{
    folders : HashMap<String, FolderItem>
}

impl FolderItem{
    fn new() -> FolderItem{ FolderItem{ folders : HashMap::new() } }

    fn set_ancestor_names(&mut self, ancs : &[String]){
        if ancs.len() == 0{ return; }
        if self.folders.contains_key(&ancs[0]) == false{
            self.folders.insert(ancs[0].clone(), FolderItem::new());
        }
        self.folders.get_mut(&ancs[0]).unwrap().set_ancestor_names(&ancs[1..]);
    }

    fn create_folders(&self, current_path : &Path){
        for (name, f) in &self.folders{
            let folder_path = current_path.join(name);

            create_dir(&folder_path).ok(); //できなければ出来ないで良い

            f.create_folders(&folder_path);
        }
    }
}