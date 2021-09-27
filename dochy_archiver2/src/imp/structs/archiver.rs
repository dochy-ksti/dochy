use std::sync::Arc;
use crate::imp::hash_thread::HashThread;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;

pub struct Archiver<T : Send + 'static>{
    hash_thread : HashThread,
    hash_receiver :Receiver<u128>,
    converter : Arc<dyn Fn(&[u8]) -> T + Send + Sync>,
    data_receivers : Vec<(String, Arc<Vec<u8>>, Receiver<T>)>
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

    pub fn archive(&mut self, path : String, data : Vec<u8>){
        let data = Arc::new(data);
        self.hash_thread.calc_hash(data.clone());

        let (sender, receiver) = mpsc::channel();
        let converter = self.converter.clone();
        let d = data.clone();
        rayon::spawn(move ||{
            let t = converter(d.as_ref());
            sender.send(t).unwrap();
        });
        self.data_receivers.push((path.clone(), data, receiver));
    }
}