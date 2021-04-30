use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex, MutexGuard};
use crate::error::FsResult;

static LOCK : Lazy<Arc<Mutex<()>>> = Lazy::new(|| Arc::new(Mutex::new(())));

pub fn lock<'a>() -> FsResult<MutexGuard<'a, ()>>{
    Ok(LOCK.as_ref().lock().map_err(|_e| Err("An error occurred in an earlier thread of this mutex"))?)
}