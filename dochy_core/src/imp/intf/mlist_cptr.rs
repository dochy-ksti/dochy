use crate::imp::structs::linked_m::{LinkedMap, LinkedMapUnsafeIter};
use crate::imp::structs::rust_list::MutItem;
use std::marker::PhantomData;
use crate::imp::structs::list_def_obj::ListDefObj;
use crate::imp::intf::mitem_ptr::MItemPtr;
use crate::imp::structs::root_def_obj::RootDefObj;
use crate::imp::intf::mitem_cptr::MItemCPtr;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MListCPtr<V : From<MItemCPtr>>{
    map : *const LinkedMap<MutItem>,
    list_def : *const ListDefObj,
    root_def : *const RootDefObj,
    phantom : PhantomData<*const V>,
}

impl<V : From<MItemCPtr>> MListCPtr<V>{
    pub fn new(map : *const LinkedMap<MutItem>, list_def : *const ListDefObj, root_def : *const RootDefObj)
        -> MListCPtr<V>{ MListCPtr { map, list_def, root_def, phantom : PhantomData } }

    fn from(&self, item : *const MutItem) -> V{
        V::from(MItemCPtr::new(item, self.list_def, self.root_def))
    }

    pub fn first(&self) -> Option<V> {
        let map = &mut *self.map;
        map.first().map(|r| self.from(r))
    }
    pub fn first_id(&self) -> Option<u64> {
        let map = unsafe{ &*self.map };
        map.first_id()
    }
    pub fn last(&self) -> Option<V> {
        let map = unsafe{ &*self.map };
        map.last().map(|r| self.from(r))
    }


    pub fn last_id(&self) -> Option<u64> {
        let map = unsafe{ &*self.map };
        map.last_id()
    }
    pub fn get_item(&self, id : u64) -> Option<V>{
        let map = unsafe{ &*self.map };
        map.get_item(id).map(|b| self.from(b))
    }

    pub fn next_id(&self) -> u64{
        let map = unsafe{ &*self.map };
        map.next_id()
    }

    pub fn contains_key(&self, key : u64) -> bool{
        let map = unsafe{ &*self.map };
        map.contains_key(key)
    }
    pub fn len(&self) -> usize{
        let map = unsafe{ &*self.map };
        map.len()
    }
    pub fn is_empty(&self) -> bool {
        let map = unsafe { &*self.map };
        map.is_empty()
    }


    pub fn iter(&self) -> MListCPtrIter<V> {
        let map = unsafe{ &*self.map };
        MListCPtrIter::new(map.iter_unsafe(), self.list_def, self.root_def)
    }


    pub fn iter_from_last(&self) -> MListCPtrIter<V> {
        let map = unsafe{ &*self.map };
        MListCPtrIter::new(map.iter_from_last_unsafe(), self.list_def, self.root_def)
    }

    pub fn iter_from_id(&self, id : u64) -> Option<MListCPtrIter<V>> {
        let map = unsafe{ &*self.map };
        map.iter_from_id_unsafe(id).map(|iter| MListCPtrIter::new(iter, self.list_def, self.root_def))
    }
}

#[derive(Debug)]
pub struct MListCPtrIter<V : From<MItemCPtr>>{
    iter : LinkedMapUnsafeIter<MutItem>,
    list_def : *const ListDefObj,
    root_def : *const RootDefObj,
    phantom : PhantomData<*mut V>,
}
impl<V : From<MItemCPtr>> Iterator for MListCPtrIter<V>{
    type Item = (u64, V);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next_mut().map(|(k,v)| (*k, V::from(MItemCPtr::new(v, self.list_def, self.root_def))))
    }
}
impl<V : From<MItemCPtr>> MListCPtrIter<V>{
    pub(crate) fn new(iter : LinkedMapUnsafeIter<MutItem>, list_def : *const ListDefObj, root_def : *const RootDefObj) -> MListCPtrIter<V>{
        MListCPtrIter { iter, list_def, root_def, phantom : PhantomData }
    }

    fn from(&self, item : *const MutItem) -> V{
        V::from(MItemCPtr::new(item, self.list_def, self.root_def))
    }
    ///現在のカーソルにあるアイテムを返し、カーソルを進める
    pub fn next(&mut self) -> Option<(u64, V)> {
        self.iter.next().map(|(k,v)| (*k, self.from(v)))
    }

    ///前に戻ることが出来る。そして元あった場所を削除し、それによって削除されたアイテムの次にあったアイテムが現在のカーソルの次にくるので、
    /// next2回でそれをとることも出来る。
    ///今ある場所をremoveしたらポインタが不正になって安全にnext/prevできない
    pub fn prev(&mut self) -> Option<(u64, V)> {
        self.iter.prev().map(|(k,v)| (*k, self.from(v)))
    }
    
    pub fn current(&mut self) -> Option<(u64, V)> {
        self.iter.current().map(|(k,v)| (*k,self.from(v)))
    }

    pub fn is_available(&self) -> bool {
        self.iter.is_available()
    }

    pub fn is_first(&self) -> bool {
        self.iter.is_first()
    }

    pub fn is_last(&self) -> bool {
        self.iter.is_last()
    }
}
