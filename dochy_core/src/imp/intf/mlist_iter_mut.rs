use crate::imp::intf::MItemPtr;
use crate::imp::intf::mlist::MListPtrIter;
use std::marker::PhantomData;
use crate::intf::mlist_mut::MItemMut;

pub struct MListIterMut<'a, V : From<MItemPtr>>{
    ptr : MListPtrIter<V>,
    phantom : PhantomData<&'a mut i32>,
}

impl<'a, V : From<MItemPtr>> Iterator for MListIterMut<'a, V>{
    type Item = (u64, MItemMut<'a, V>);

    fn next(&mut self) -> Option<Self::Item> {
        self.ptr.next().map(|(id, v)| (id, MItemMut::from_phantom(v, self.phantom)))
    }
}
impl<'a, V : From<MItemPtr>> MListIterMut<'a, V>{
    pub fn next(&mut self) -> Option<(u64, MItemMut<'a, V>)> {
        self.ptr.next().map(|(id, v)| (
            id,
            MItemMut::from_phantom(v, self.phantom)))
    }
    pub fn prev(&mut self) -> Option<(u64, MItemMut<'a, V>)> {
        self.ptr.next().map(|(id, v)| (
            id,
            MItemMut::from_phantom(v, self.phantom)))
    }
    pub fn current(&mut self) -> Option<(u64, MItemMut<'a, V>)> {
        self.ptr.current().map(|(id, v)| (
            id,
            MItemMut::from_phantom(v, self.phantom)))
    }
    pub fn is_available(&self) -> bool {
        self.ptr.is_available()
    }
    pub fn is_first(&self) -> bool {
        self.ptr.is_first()
    }
    pub fn is_last(&self) -> bool {
        self.ptr.is_last()
    }
}