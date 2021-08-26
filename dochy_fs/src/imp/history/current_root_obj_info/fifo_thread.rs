use parking_lot::Mutex;
use rayon::{ThreadPool, ThreadPoolBuilder};

pub(crate) struct FifoThread{
    pool : Mutex<Option<ThreadPool>>,
}

impl FifoThread{
    pub fn new() -> FifoThread{
        FifoThread{ pool : Mutex::new(None) }
    }

    pub fn spawn_fifo<F : FnOnce() + Send + 'static>(&self, f : F){
        let mut opt = self.pool.lock();
        if opt.is_none(){
            *opt = Some(ThreadPoolBuilder::new().num_threads(1).build().unwrap())
        }
        if let Some(p) = opt.as_ref(){
            p.spawn_fifo(f);
        }
    }
}