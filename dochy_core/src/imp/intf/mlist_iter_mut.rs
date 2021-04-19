use crate::imp::intf::MItemPtr;
use crate::imp::intf::mlist::MListPtrIter;
use std::marker::PhantomData;
use crate::imp::intf::mlist_mut::MItemMut;

pub struct MListIterMut<'a, V : From<MItemPtr>>{
    ptr : MListPtrIter<V>,
    phantom : PhantomData<&'a mut i32>,
}

impl<'a,  V : From<MItemPtr>> Iterator for MListIterMut<'a, V>{
    type Item = (u64, MItemMut<'a, V>);

    fn next(&mut self) -> Option<Self::Item> {
        self.ptr.next().map(|(id,a)| (id, MItemMut::from_phantom(a,  self.phantom)))
    }
}

impl<'a, V : From<MItemPtr>> MListIterMut<'a, V>{

}

