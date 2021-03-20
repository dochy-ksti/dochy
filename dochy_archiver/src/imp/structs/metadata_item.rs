use std::time::SystemTime;
use crate::imp::structs::file_item::FileItem;
use crate::ArcResult;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq)]
pub struct MetadataItem{
    relative_path: String,
    modified : SystemTime,
    len : u64,
}

impl MetadataItem{
    pub(crate) fn new(relative_path : String, modified : SystemTime, len : u64) -> Self{ Self{ relative_path, modified, len }}
    //pub(crate) fn json5_ext() -> &'static str{ "json5_back" }

    pub(crate) fn from_file_item(file_item : &FileItem, root_path : &Path) -> ArcResult<MetadataItem>{
        let path = file_item.file_path();
        let relative_path = path.strip_prefix(root_path)?;
        let relative_path = relative_path.to_string_lossy().to_string();
        Ok(Self::new(relative_path, file_item.modified().clone(), file_item.len()))
    }

    ///The file's relative path from the archive root.
    pub fn relative_path(&self) -> &str{ &self.relative_path }

    /// The modified time of the file.
    /// When the modified time is changed, this library considers the file is changed.
    pub fn modified(&self) -> &SystemTime{ &self.modified }

    /// The file's length. Even if the modified time is unchanged,
    /// if the len is changed, this library considers the file is changed too.
    pub fn len(&self) -> u64{ self.len }

    pub fn calc_full_path(&self, dir_path : impl AsRef<Path>) -> PathBuf{
        dir_path.as_ref().join(self.relative_path())
    }
}
