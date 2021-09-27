// use std::time::SystemTime;
// use std::path::PathBuf;
//
// pub(crate) struct FileItem {
//     file_path : PathBuf,
//     modified : SystemTime,
//     len : u64,
// }
//
// impl FileItem {
//     pub(crate) fn new(file_path : PathBuf, modified : SystemTime, len : u64) -> FileItem{ Self{ file_path, modified, len } }
//     pub(crate) fn file_path(&self) -> &PathBuf{ &self.file_path }
//     pub(crate) fn modified(&self) -> &SystemTime{ &self.modified }
//     pub(crate) fn len(&self) -> u64{ self.len }
//     //pub(crate) fn deconstruct(self) -> (PathBuf, SystemTime, u64){ (self.file_path, self.modified, self.len) }
// }
