use crate::imp::intf::MItemPtr;
use crate::imp::intf::MListPtr;
use std::marker::PhantomData;

pub struct MListMut<'a, V : MListMutItemTrait, T>{
    ptr : MListPtr<V::PtrItem>,
    src : &'a mut T,
    phantom2 : PhantomData<*const V>,
}

impl<'a, V : MListMutItemTrait, T> MListMut<'a, V, T>{
    pub fn new(ptr : MListPtr<V::PtrItem>, src : &mut T) -> MListMut<V, T>{
        MListMut{ ptr, src, phantom2 : PhantomData::default() }
    }

    pub fn src(&'a mut self) -> &'a mut T{ self.src }

    pub fn first(&mut self) -> Option<V>{
        self.ptr.first().map(|v| V::from(v, self))
    }
}

pub trait MListMutItemTrait{
    type PtrItem : From<MItemPtr>;

    fn from<T>(
        ptr_item : Self::PtrItem,
        src : &mut MListMut<Self, T>) -> Self where Self : Sized;
}