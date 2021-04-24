use std::marker::PhantomData;
use std::ops::Deref;

#[derive(Debug)]
pub struct CTableConst<'a, T>{
    ptr : T,
    phantom : PhantomData<&'a i32>,
}

impl<'a, T> CTableConst<'a, T>{
    pub fn new<U>(ptr : T, _src : &'a U) -> CTableConst<'a, T>{
        CTableConst{ ptr, phantom : PhantomData }
    }
}

impl<'a, T> Deref for CTableConst<'a, T>{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.ptr
    }
}