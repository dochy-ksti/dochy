use std::sync::Arc;
use crate::imp::structs::hash_thread::HashThread;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use crate::imp::structs::archive_data::{ArchiveData, ArchiveDataItem};
use std::collections::BTreeMap;
use crate::ArcResult;

pub(crate) struct Archiver<T : Send + 'static>{
    hash_thread : HashThread,
    hash_receiver :Receiver<u128>,
    converter : Arc<dyn Fn(&[u8]) -> T + Send + Sync>,
    data_receivers : Vec<ArchiverItem<T>>,
}

pub(crate) struct ArchiverItem<T : Send + 'static>{
    path : String,
    raw_data : Arc<Vec<u8>>,
    processed : Receiver<T>,
}


impl<T : Send + 'static> Archiver<T>{

    pub fn new(f : Arc<dyn Fn(&[u8]) -> T + Send + Sync>) -> Archiver<T>{
        let (sender, receiver) = mpsc::channel();
        let hash_thread = HashThread::new(sender);

        Archiver{
            hash_thread,
            hash_receiver : receiver,
            converter : f,
            data_receivers : Vec::new(),
        }
    }

    /// path をアルファベット順にして この fn を何度も呼び出すべし。スレッドに流されて非同期に実行される
    pub fn archive(&mut self, path : String, data : Vec<u8>){
        let data = Arc::new(data);
        self.hash_thread.calc_hash(data.clone());

        let (sender, processed) = mpsc::channel();
        let converter = self.converter.clone();
        let d = data.clone();
        rayon::spawn(move ||{
            let t = converter(d.as_ref());
            sender.send(t).unwrap();
        });

        // let (sender, compressed) = mpsc::channel();
        // let d = data.clone();
        //
        // rayon::spawn(move ||{
        //     let mut encoder = snap::raw::Encoder::new();
        //     let vec = encoder.compress_vec(d.as_slice()).unwrap();
        //     sender.send(vec).unwrap();
        // });

        self.data_receivers.push(ArchiverItem{
            path,
            raw_data : data,
            processed
        });
    }

    pub fn finish(self) -> ArcResult<ArchiveData<T>>{
        let mut btree : BTreeMap<String, ArchiveDataItem<T>> = BTreeMap::new();
        let hash = self.hash_receiver.recv()?;
        for item in self.data_receivers {

            let processed = item.processed.recv()?;
            let path = item.path;
            let item = ArchiveDataItem::new(processed, item.raw_data);

            btree.insert(path, item);
        }
        Ok(ArchiveData::new(btree, hash))
    }
}