use crate::structs::{RootObject, RootValue, RootSabValue};
use crate::error::CoreResult;
use dochy_archiver2::ArchiveData;

pub struct DochyArchive{
    pub(crate) data : ArchiveData<CoreResult<ArchivingItem>>
}

impl DochyArchive{
    pub fn hash(&self) -> u128{
        self.data.hash()
    }
}

pub(crate) enum ArchivingItem{
    Root(RootObject),
    Item((String, RootValue, Option<RootSabValue>)),
}