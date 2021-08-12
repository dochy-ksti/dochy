use crate::error::FsResult;
use std::io::{Read, Write};
use crate::imp::history::diff_and_cache::diff_src::DiffSrc;

pub(crate) trait DiffValue : Sized{
    fn read_value<R : Read>(read : &mut R) -> FsResult<Self>;
    fn write_value<W : Write>(&self, write : &mut W) -> FsResult<()>;
}

