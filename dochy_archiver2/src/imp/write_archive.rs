use crate::{ArchiveData, ArcResult};
use dochy_compaction::kval_enum::KVal;
use std::collections::BTreeMap;
use std::sync::mpsc::channel;
use dochy_compaction::basic_compaction::{comp_str, comp_int};
use std::io::Write;
use crate::imp::structs::arc_write_item::ArcWriteItem;
use crate::imp::write_items::write_items;


pub fn write_archive<W : Write, T : Send + 'static>(writer : &mut W, data : ArchiveData<T>) -> ArcResult<()>{
    let mut items : BTreeMap<String, Vec<u8>> = BTreeMap::new();;

    let (sender, receiver) = channel();

    for (key,val) in data.btree(){
        let key = key.to_string();
        let raw_data = val.raw_data().clone();
        let sender = sender.clone();
         rayon::spawn(move||{
             let mut encoder = snap::raw::Encoder::new();
             match encoder.compress_vec(raw_data.as_slice()){
                 Ok(a) => {
                     sender.send(ArcWriteItem {
                         path: key,
                         compressed: a,
                         raw_len: raw_data.len(),
                     }).unwrap();
                 },
                 Err(e) =>{
                     // 4GB超えでErrorらしい
                     panic!("{}", e);
                 }
             }
         })
    }

    for _ in 0..data.btree().len(){
        let item = receiver.recv().unwrap();
        items.insert(item.path, item.compressed);
    }

    write_items(items, writer)
}