use std::hash::Hasher;
use metrohash::MetroHash128;
use std::sync::{Arc};
use std::sync::mpsc::{Sender, channel, Receiver};

pub(crate) struct HashThread{
    vec_sender : Sender<Option<Arc<Vec<u8>>>>,
    result_receiver : Receiver<u128>,
}

impl HashThread{
    pub fn new() -> HashThread{
        let (vec_sender, vec_receiver) = channel();
        let (result_sender, result_receiver) = channel();
        std::thread::spawn(move || {
            let mut hasher = MetroHash128::new();
            loop{
                match vec_receiver.recv().unwrap(){
                    Some(v) =>{
                        let v : Arc<Vec<u8>> = v;
                        hasher.write(v.as_ref())
                    },
                    None => break,
                }
            }
            let (l,r) = hasher.finish128();
            result_sender.send(to_u128(l,r)).unwrap();
        });
        HashThread{
            vec_sender,
            result_receiver,
        }
    }

    pub fn calc_hash(&mut self, vec : Arc<Vec<u8>>){
        self.vec_sender.send(Some(vec)).unwrap();
    }

    pub fn finish(&self) -> u128{
        self.vec_sender.send(None).unwrap();
        self.result_receiver.recv().unwrap()
    }
}

fn to_u128(l : u64, r : u64) -> u128{
    let mut r = u128::from(r);
    r |= (l as u128) << 64;
    r
}