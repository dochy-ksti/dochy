use crate::imp::intf::MItemPtr;
use crate::imp::intf::MListPtr;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct MListMut<'a, V : From<MItemPtr>>{
    ptr : MListPtr<V>,
    phantom : PhantomData<&'a mut i32>,
}

impl<'a, V : From<MItemPtr>> MListMut<'a, V>{
    pub fn new<T>(ptr : MListPtr<V>, _src : &'a mut T) -> MListMut<'a, V>{
        MListMut{ ptr, phantom : PhantomData }
    }

    pub fn first(&mut self) -> Option<MItemMut<V>>{
        self.ptr.first().map(
            move |v| MItemMut::new(v, self))
    }


}

pub struct MItemMut<'a, V>{
    item : V,
    phantom : PhantomData<&'a mut i32>,
}

impl<'a, V> MItemMut<'a, V>{
    pub fn new<T>(item : V, _src : &'a mut T) -> MItemMut<'a, V>{
        MItemMut{ item, phantom : PhantomData }
    }
}

impl<'a, T> Deref for MItemMut<'a, T>{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.item
    }
}

impl<'a, T> DerefMut for MItemMut<'a, T>{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.item
    }
}