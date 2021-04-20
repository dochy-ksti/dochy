use crate::imp::intf::MItemPtr;
use crate::imp::intf::MListPtr;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use crate::imp::intf::mlist_const::MItemConst;

#[derive(Debug)]
pub struct MListMut<'a, V : From<MItemPtr>>{
    ptr : MListPtr<V>,
    phantom : PhantomData<&'a mut i32>,
}

impl<'a, V : From<MItemPtr>> MListMut<'a, V>{
    pub fn new<T>(ptr : MListPtr<V>, _src : &'a mut T) -> MListMut<'a, V>{
        MListMut{ ptr, phantom : PhantomData }
    }

    pub fn first(&self) -> Option<MItemConst<V>>{
        unsafe{ self.ptr.first_const() }.map(
            move |v| MItemConst::new(v, self))
    }
    pub fn first_mut(&mut self) -> Option<MItemMut<V>>{
        self.ptr.first().map(
            move |v| MItemMut::new(v, self))
    }
    pub fn first_id(&self) -> Option<u64>{
        self.ptr.first_id()
    }
    pub fn last(&self) -> Option<MItemConst<V>>{
        unsafe{ self.ptr.last_const() }.map(
            move |v| MItemConst::new(v, self))
    }
    pub fn last_id(&self) -> Option<u64>{
        self.ptr.last_id()
    }
    pub fn get_item(&self, id : u64) -> Option<MItemConst<V>>{
        unsafe{ self.ptr.get_item_const(id) }.map(
            move |v| MItemConst::new(v, self))
    }
    pub fn next_id(&self) -> u64{
        self.ptr.next_id()
    }
    pub fn contains_key(&self, key : u64) -> bool{
        self.ptr.contains_key(key)
    }
    pub fn len(&self) -> usize{
        self.ptr.len()
    }
    pub fn is_empty(&self) -> bool{
        self.ptr.is_empty()
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
    pub fn from_phantom<T>(item : V, _src : PhantomData<&'a mut T>) -> MItemMut<'a, V>{
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