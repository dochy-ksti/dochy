use crate::imp::intf::citem::CItemPtr;
use crate::imp::structs::list_def_obj::ListDefObj;
use std::marker::PhantomData;
use crate::imp::structs::rust_list::ConstItem;
use crate::imp::structs::root_def_obj::RootDefObj;

/// CList's internal structure is Vec
#[derive(Debug, PartialEq)]
pub struct CListPtr<'a, T : From<CItemPtr>> {
    ptr : &'a Vec<ConstItem>,
    list_def : &'a ListDefObj,
    root_def : &'a RootDefObj,
    phantom : PhantomData<*mut T>,
}
impl<'a, T : From<CItemPtr<'a>>> Clone for CListPtr<'a, T>{
    fn clone(&self) -> Self {
        CListPtr::new(self.ptr, self.list_def, self.root_def)
    }
}
impl<'a, T : From<CItemPtr<'a>>> Copy for CListPtr<'a, T>{}

impl<'a, T : From<CItemPtr<'a>>> CListPtr<'a, T> {
    pub fn new<'b>(ptr : &'b Vec<ConstItem>, list_def : &'b ListDefObj, root_def : &'b RootDefObj) -> CListPtr<'b, T> { CListPtr { ptr, list_def, root_def, phantom : PhantomData } }
    pub fn len(&self) -> usize{ self.ptr.len() }
    pub fn value(&self, idx : usize) -> T{ get_value(self.clone(), idx) }
    pub fn iter(&self) -> CListPtrIter<T>{ get_iter(self.clone()) }
}


pub fn get_value<'a, T: From<CItemPtr<'a>>>(list: CListPtr<T>, idx : usize) -> T{
    let vec = list.ptr;
    T::from(CItemPtr::new(&vec[idx], list.list_def, list.root_def))
}

pub fn get_iter<'a, T : From<CItemPtr<'a>>>(list: CListPtr<T>) -> CListPtrIter<T>{
    CListPtrIter::new(list.ptr, list.list_def, list.root_def)
}

pub struct CListPtrIter<'a, V : From<CItemPtr<'a>>>{
    vec : &'a Vec<ConstItem>,
    list_def : &'a ListDefObj,
    root_def : &'a RootDefObj,
    index : usize,
    phantom : PhantomData<*mut V>,
}
impl<'a, V : From<CItemPtr<'a>>> Iterator for CListPtrIter<'a, V>{
    type Item = V;

    fn next(&mut self) -> Option<Self::Item> {
        let vec = self.vec;
        if self.index < vec.len(){
            let index = self.index;
            self.index += 1;
            Some(V::from(CItemPtr::new(vec.get(index).unwrap(), self.list_def, self.root_def)))
        } else{
            None
        }

    }
}
impl<'a, V : From<CItemPtr<'a>>> CListPtrIter<'a, V>{
    pub fn new<'b>(vec : &'b Vec<ConstItem>, list_def : &'b ListDefObj, root_def : &'b RootDefObj) -> CListPtrIter<'b, V>{
        CListPtrIter { vec, list_def, root_def, index : 0, phantom : PhantomData }
    }
}
