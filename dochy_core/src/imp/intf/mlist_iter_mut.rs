use crate::imp::intf::MItemPtr;
use crate::imp::intf::mlist::MListPtrIter;
use std::marker::PhantomData;

pub struct MListIterMut<'a, V : From<MItemPtr>>{
    ptr : MListPtrIter<V>,
    phantom : PhantomData<&'a mut i32>,
}