use once_cell::sync::Lazy;
use std::sync::{Mutex, MutexGuard, PoisonError};

static LOCK : Lazy<Mutex<()>> = Lazy::new(||Mutex::new(()));

pub(crate) fn try_lock_mutex<'a>() -> Result<MutexGuard<'a, ()>, PoisonError<MutexGuard<'a, ()>>>{
    LOCK.lock()
}

pub(crate) fn lock_mutex<'a>() -> MutexGuard<'a, ()>{
    try_lock_mutex().unwrap_or_else(|e| e.into_inner())
}