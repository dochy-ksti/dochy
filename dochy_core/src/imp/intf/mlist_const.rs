use crate::imp::intf::MItemPtr;
use crate::imp::intf::MListPtr;
use std::marker::PhantomData;
use std::ops::{Deref};

#[derive(Debug)]
pub struct MListConst<'a, V : From<MItemPtr>>{
    ptr : MListPtr<V>,
    phantom : PhantomData<&'a i32>,
}

impl<'a, V : From<MItemPtr>> MListConst<'a, V>{
    pub fn new<T>(ptr : MListPtr<V>, _src : &'a T) -> MListConst<'a, V>{
        MListConst{ ptr, phantom : PhantomData }
    }

    pub fn first(&self) -> Option<MItemConst<V>>{
        unsafe{ self.ptr.first_const() }.map(
            move |v| MItemConst::new(v, self))
    }


}

pub struct MItemConst<'a, V>{
    item : V,
    phantom : PhantomData<&'a i32>,
}

impl<'a, V> MItemConst<'a, V>{
    pub fn new<T>(item : V, _src : &'a T) -> MItemConst<'a, V>{
        MItemConst{ item, phantom : PhantomData }
    }
}

impl<'a, T> Deref for MItemConst<'a, T>{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.item
    }
}