use parking_lot::MutexGuard;
use crate::imp::filesys::save_cache_item::SaveCacheItem;
use std::sync::atomic::Ordering;

pub(crate) struct DochyMutex<'a>{
    guard : MutexGuard<'a, ()>,
    cache: &'a SaveCacheItem,
}
impl<'a> DochyMutex<'a>{
    pub(crate) fn new<'b>(guard : MutexGuard<'b, ()>,
                          cache : &'b SaveCacheItem) -> DochyMutex<'b>{
        cache.queued_atomic().fetch_add(1, Ordering::Relaxed);
        DochyMutex { guard, cache }
    }
    pub(crate) fn cache(&self) -> &SaveCacheItem{ self.cache }
}
impl<'a> Drop for DochyMutex<'a>{
    fn drop(&mut self) {
        self.cache().queued_atomic().fetch_sub(1, Ordering::Relaxed);
    }
}