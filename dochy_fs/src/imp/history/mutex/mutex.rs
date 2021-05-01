use once_cell::sync::Lazy;
use std::sync::{Mutex, MutexGuard, PoisonError};

static LOCK : Lazy<Mutex<()>> = Lazy::new(||Mutex::new(()));

pub fn lock<'a>() -> Result<MutexGuard<'a, ()>, PoisonError<MutexGuard<'a, ()>>>{
    LOCK.lock()
}

