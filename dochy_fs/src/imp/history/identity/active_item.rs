use crate::imp::history::file_name::file_name_props::FileNameProps;
use dochy_core::structs::RustIdentity;

pub(crate) struct ActiveItem{
    parent_file_name : FileNameProps,
    identity : RustIdentity
}

impl ActiveItem{
    pub fn parent_file_name(&self) -> &FileNameProps{
        &self.parent_file_name
    }

    pub fn identity(&self) -> &RustIdentity{
        &self.identity
    }
}