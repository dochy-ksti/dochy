use std::collections::{BTreeMap};
use crate::archiver::imp::structs::metadata::Metadata;

#[derive(Debug)]
pub struct ArchiveData{
    meta : Metadata,
    file_data : BTreeMap<String, Vec<u8>>
}

impl ArchiveData{
    pub(crate) fn new(meta : Metadata, file_data : BTreeMap<String, Vec<u8>>) -> Self{ Self{ meta, file_data } }
    //pub(crate) fn deconstruct(self) -> (Metadata, BTreeMap<String, String>){ (self.meta, self.file_data) }

    /// The metadata of the archive
    pub fn meta(&self) -> &Metadata{ &self.meta }

    /// Get data from the relative path
    pub fn get_data(&self, relative_path : &str) -> Option<&[u8]>{
        self.file_data.get(relative_path).map(|v| v.as_slice())
    }

    /// Iterate each relative path and data
    pub fn iter(&self) -> impl Iterator<Item=(&str, &[u8])>{
        self.file_data.iter().map(|(key,val)| (key.as_str(), val.as_slice()))
    }
}