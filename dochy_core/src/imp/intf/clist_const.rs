use crate::imp::intf::{CItemPtr, CListPtr};
use std::marker::PhantomData;
use crate::imp::intf::clist_iter_const::CListIterConst;
use std::ops::Deref;

#[derive(Debug)]
pub struct CListConst<'a, V : From<CItemPtr>>{
    ptr : CListPtr<V>,
    phantom : PhantomData<&'a i32>,
}

impl<'a, V : From<CItemPtr>> CListConst<'a, V>{
    pub fn new<T>(ptr : CListPtr<V>, _src : &'a T) -> CListConst<'a, V>{
        CListConst{ ptr, phantom : PhantomData }
    }

    pub fn len(&self) -> usize{
        self.ptr.len()
    }
    pub fn value(&self, idx : usize) -> CItemConst<'a, V>{
        CItemConst::from_phantom(self.ptr.value(idx), self.phantom)
    }

    pub fn iter(&self) -> CListIterConst<V>{
        CListIterConst::new(self.ptr.iter(), self)
    }
}

pub struct CItemConst<'a, V>{
    item : V,
    phantom : PhantomData<&'a i32>,
}

impl<'a, V> CItemConst<'a, V>{
    pub fn new<T>(item : V, _src : &'a T) -> CItemConst<'a, V>{
        CItemConst{ item, phantom : PhantomData }
    }
    pub fn from_phantom<T>(
        item : V,
        _src : PhantomData<&'a T>) -> CItemConst<'a, V>{
        CItemConst{ item, phantom : PhantomData }
    }
}

impl<'a, T> Deref for CItemConst<'a, T>{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.item
    }
}