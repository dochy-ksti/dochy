use std::sync::Arc;
use crate::imp::hash_thread::HashThread;

pub struct Archiver{
    hash_thread : HashThread,
}

impl Archiver{
    pub fn archive(&mut self, path : String, data : Vec<u8>){
        let data = Arc::new(data);
    }
}