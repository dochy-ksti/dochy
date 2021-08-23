use crate::error::FsResult;
use std::thread::JoinHandle;

pub struct JoinHandler<T>{
    handle : JoinHandle<T>
}

impl<T> JoinHandler<T>{
    pub fn join(self) -> FsResult<T>{
        Ok(self.handle.join().or(Err("join failed"))?)
    }

    pub(crate) fn new(handle : JoinHandle<T>) -> Self{
        Self{ handle }
    }
}