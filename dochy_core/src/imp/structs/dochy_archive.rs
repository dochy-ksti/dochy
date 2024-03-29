use crate::structs::{RootObject, RootValue, RootSabValue};
use crate::error::CoreResult;
use dochy_archiver2::{ArchiveData, write_archive};
use std::io::Write;
use crate::imp::json_to_rust::roots::archive_data_to_root::archive_to_root;

pub struct DochyArchive{
    pub(crate) data : ArchiveData<CoreResult<ArchivingItem>>
}

impl DochyArchive{
    pub(crate) fn new(data : ArchiveData<CoreResult<ArchivingItem>>) -> DochyArchive{
        DochyArchive{ data }
    }
    pub fn hash(&self) -> u128{
        self.data.hash()
    }
    pub fn write_archive<W : Write>(&self, writer : &mut W) -> CoreResult<()>{
        Ok(write_archive(&self.data, writer)?)
    }
    pub fn into_root(self, validation : bool) -> CoreResult<RootObject>{
        archive_to_root(self, validation)
    }
}

pub(crate) enum ArchivingItem{
    Root(RootObject),
    Item((String, RootValue, Option<RootSabValue>)),
}