use crate::{ArchiveData, ArcResult};
use dochy_compaction::kval_enum::KVal;
use std::collections::BTreeMap;
use std::sync::mpsc::{channel, Sender};
use dochy_compaction::basic_compaction::{comp_str, comp_int};
use std::io::Write;
use crate::imp::structs::arc_write_item::ArcWriteItem;
use crate::imp::write_items::write_items;


pub fn write_archive<W : Write, T : Send + 'static>(writer : &mut W, data : ArchiveData<T>) -> ArcResult<()>{
    let mut items : BTreeMap<String, Vec<u8>> = BTreeMap::new();;

    let (sender , receiver) = channel();

    for (key,val) in data.btree(){
        let key = key.to_string();
        let raw_data = val.raw_data().clone();
        let sender : Sender<ArcResult<(String, Vec<u8>)>> = sender.clone();
         rayon::spawn(move||{
             let mut encoder = snap::raw::Encoder::new();
             match encoder.compress_vec(raw_data.as_slice()){
                 Ok(compressed) => {
                     sender.send(Ok((key, compressed))).unwrap();
                 },
                 Err(e) =>{
                     sender.send(Err(format!("{}", e)).into());
                 }
             }
         })
    }

    for _ in 0..data.btree().len(){
        let (path, compressed) = receiver.recv().unwrap()?;
        items.insert(path, compressed);
    }

    write_items(items, writer)
}