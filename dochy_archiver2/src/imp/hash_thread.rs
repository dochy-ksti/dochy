use std::hash::Hasher;
use metrohash::MetroHash128;
use std::sync::Arc;
use std::cell::RefCell;
use std::sync::mpsc::Sender;

pub(crate) struct HashThread{
    thread : rayon::ThreadPool,
    hasher : Arc<RefCell<MetroHash128>>,
    sender : Arc<Sender<u128>>,
}

impl HashThread{
    pub fn new(sender : Sender<u128>) -> HashThread{
        HashThread{
            thread : rayon::ThreadPoolBuilder::new().num_threads(1).build().unwrap(),
            hasher : Arc::new(RefCell::new(MetroHash128::new())),
            sender : Arc::new(sender),
        }
    }

    pub fn calc_hash(&mut self, vec : Arc<Vec<u8>>){
        let hasher = self.hasher.clone();
        self.thread.spawn_fifo(move ||{
            hasher.borrow_mut().write(vec.as_ref())
        })
    }

    pub fn finish(&mut self){
        let hasher = self.hasher.clone();
        let sender = self.sender.clone();
        self.thread.spawn_fifo(||{
            let (l,r) = hasher.borrow().finish128();
            let mut r = u128::from(r);
            r |= (l as u128) << 64;
            sender.send(r);
        })
    }
}
pub(crate) fn calc_hash(data : &[u8]) -> u128{
    let mut hasher = metrohash::MetroHash128::new();
    hasher.write(data);
    let (l,r) = hasher.finish128();
    let mut r = u128::from(r);
    r |= (l as u128) << 64;
    r
}