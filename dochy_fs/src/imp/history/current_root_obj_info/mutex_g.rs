use crate::history::PeekableCacheInfo;
use std::ops::{Deref, DerefMut};
use crate::imp::history::current_root_obj_info::history_cache_item::SyncedItem;
use parking_lot::MutexGuard;
use std::path::PathBuf;
use std::sync::atomic::Ordering;

pub(crate) struct MutexG<'a>{
    guard : MutexGuard<'a, SyncedItem>,
    history_dir : PathBuf,
    peekable_info : &'a PeekableCacheInfo,
}
impl<'a> MutexG<'a>{
    pub(crate) fn new<'b>(guard : MutexGuard<'b, SyncedItem>,
                      history_dir : PathBuf,
                      peekable_info : &'b PeekableCacheInfo) -> MutexG<'b>{
        MutexG{ guard, history_dir, peekable_info }
    }
    //pub fn history_dir(&self) -> &Path{ &self.peekable_info. }
    pub(crate) fn peekable(&self) -> &PeekableCacheInfo{ &self.peekable_info }
    //pub(crate) fn set_current_root_obj_info(&mut self, current_root_obj_info : Option<CurrentRootObjInfo>) {
    //  *self.guard.current_root = current_root_obj_info;
    //}
}
impl<'a> Drop for MutexG<'a>{
    fn drop(&mut self) {
        self.peekable().queued_atomic().fetch_sub(1, Ordering::Relaxed);
    }
}

impl<'a> Deref for MutexG<'a>{
    type Target = SyncedItem;

    fn deref(&self) -> &Self::Target {
        Deref::deref(&self.guard)
    }
}

impl<'a> DerefMut for MutexG<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        DerefMut::deref_mut(&mut self.guard)
    }
}