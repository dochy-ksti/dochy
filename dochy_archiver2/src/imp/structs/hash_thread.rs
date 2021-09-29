use std::hash::Hasher;
use metrohash::MetroHash128;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::Sender;

pub(crate) struct HashThread{
    thread : rayon::ThreadPool,
    hasher : Arc<Mutex<MetroHash128>>,
    sender : Sender<u128>,
}

impl HashThread{
    pub fn new(sender : Sender<u128>) -> HashThread{
        HashThread{
            thread : rayon::ThreadPoolBuilder::new().num_threads(1).build().unwrap(),
            hasher : Arc::new(Mutex::new(MetroHash128::new())),
            sender,
        }
    }

    pub fn calc_hash(&mut self, vec : Arc<Vec<u8>>){
        let hasher = self.hasher.clone();
        self.thread.spawn_fifo(move ||{
            let mut locked = hasher.lock().unwrap();
            locked.write(vec.as_ref())
        })
    }

    pub fn finish(&mut self){
        let hasher = self.hasher.clone();
        let sender = self.sender.clone();
        self.thread.spawn_fifo(move ||{
            let locked = hasher.lock().unwrap();
            let (l,r) = locked.finish128();
            let mut r = u128::from(r);
            r |= (l as u128) << 64;
            sender.send(r).unwrap();
        })
    }
}