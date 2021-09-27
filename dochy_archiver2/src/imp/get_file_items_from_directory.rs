use std::path::Path;
use crate::{ArcResult, ArchiveOptions};
use crate::imp::structs::file_item_b::FileItem;

pub(crate) fn get_file_items_from_directory<P : AsRef<Path>>(path : P, opt : &ArchiveOptions) -> ArcResult<Vec<FileItem>>{
    let mut r : Vec<FileItem> = vec![];
    get_items_from_dir(path.as_ref(), opt, &mut r)?;
    Ok(r)
}

fn get_items_from_dir(current_path : &Path, opt : &ArchiveOptions, r : &mut Vec<FileItem>) -> ArcResult<()>{
    let dirs = std::fs::read_dir(current_path)?;

    for dir in dirs {
        let de = dir?;

        let meta = de.metadata()?;
        if meta.is_file() {
            let path = de.path();
            let ext = path.extension().map_or("", |e| e.to_str().unwrap_or(""));
            if opt.is_archived(ext){
                let modified = meta.modified()?;
                let len = meta.len();
                r.push(FileItem::new(path, modified, len))
            }
        } else if meta.is_dir(){
            if opt.archive_subfolders(){
                get_items_from_dir(&de.path(), opt, r)?;
            }
        }
    }

    Ok(())
}