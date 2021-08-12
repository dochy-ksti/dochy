use crate::imp::history::diff_and_cache::diff_value::{DiffValue};
use crate::error::FsResult;
use std::io::{Read, Write};

pub(crate) struct DochyDiff{
    vec : Vec<u8>
}

impl DiffValue for DochyDiff{
    fn read_value<R: Read>(read: &mut R) -> FsResult<Self> {
        let mut vec : Vec<u8> = vec![];
        read.read_to_end(&mut vec)?;
        Ok(DochyDiff{ vec })
    }

    fn write_value<W: Write>(&self, write: &mut W) -> FsResult<()> {
        write.write_all(&self.vec)?;
        Ok(())
    }
}

impl DochyDiff{
    pub(crate) fn new(vec : Vec<u8>) -> DochyDiff{ DochyDiff{ vec } }
    pub(crate) fn slice(&self) -> &[u8]{ &self.vec }
}
