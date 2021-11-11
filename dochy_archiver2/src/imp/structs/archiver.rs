use std::sync::Arc;
use crate::imp::structs::hash_thread::HashThread;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use crate::imp::structs::archive_data::{ArchiveData, ArchiveDataItem};
use std::collections::BTreeMap;
use crate::ArcResult;

pub(crate) struct Archiver<T : Send + 'static>{
    hash_thread : HashThread,
    converter : Arc<dyn Fn(&str, &[u8]) -> T + Send + Sync + 'static>,
    data_receivers : Vec<ArchiverItem<T>>,
}

pub(crate) struct ArchiverItem<T : Send + 'static>{
    path : String,
    raw_data : Arc<Vec<u8>>,
    processed : Receiver<T>,
}


impl<T : Send + 'static> Archiver<T>{

    pub fn new(f : impl Fn(&str, &[u8]) -> T + Send + Sync + 'static) -> Archiver<T>{

        let hash_thread = HashThread::new();
        Archiver{
            hash_thread,
            converter : Arc::new(f),
            data_receivers : Vec::new(),
        }
    }

    /// path をアルファベット順にして この fn を何度も呼び出すべし。スレッドに流されて非同期に実行される
    pub fn archive(&mut self, path : String, data : Vec<u8>){
        let data = Arc::new(data);
        println!("path {}", &path);
        self.hash_thread.calc_hash(path.clone(), data.clone());
        println!("calchashed");
        let (sender, processed) = mpsc::channel();
        println!("calchashed2");
        let converter = self.converter.clone();
        println!("calchashed3");
        let d = data.clone();
        println!("calchashed4");
        let p = path.clone();
        println!("calchashed5");

        println!("rayon threads {}", rayon::current_num_threads());
        println!("rayon threads {}", rayon::current_num_threads());
        rayon::spawn_fifo(move ||{
            println!("archive spawn");
            let t = converter(&p, d.as_ref());
            println!("spawn");
            sender.send(t).ok();
            println!("send");
        });

        self.data_receivers.push(ArchiverItem{
            path,
            raw_data : data,
            processed
        });
    }

    pub fn finish(self) -> ArcResult<ArchiveData<T>>{
        let mut btree : BTreeMap<String, ArchiveDataItem<T>> = BTreeMap::new();

        for item in self.data_receivers {
            let processed = item.processed.recv()?;
            println!("recv");
            let path = item.path;
            let item = ArchiveDataItem::new(processed, item.raw_data);

            btree.insert(path, item);
            println!("insert")
        }
        let hash = self.hash_thread.finish()?;
        println!("finished");

        Ok(ArchiveData::new(btree, hash))
    }
}