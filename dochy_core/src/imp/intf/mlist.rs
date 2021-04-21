use crate::imp::structs::linked_m::{LinkedMap, LinkedMapUnsafeIter};
use crate::imp::structs::rust_list::MutItem;
use std::marker::PhantomData;
use crate::imp::structs::list_def_obj::ListDefObj;
use crate::imp::structs::root_obj::RootObject;
use crate::imp::intf::mitem::MItemPtr;

/// This uses pointers so every method is basically unsafe.
/// You can get this ptr, and create an immutable reference,
/// and modify the referent through this pointer,
/// and access the immutable reference afterwards.
/// Anything can happen with the access.
///
/// getting data through this pointer while a mutable reference is alive
/// is also an undefined behavior.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MListPtr<V : From<MItemPtr>>{
    map : *mut LinkedMap<MutItem>,
    list_def : *const ListDefObj,
    root : *mut RootObject,
    phantom : PhantomData<*const V>,
}

impl<V : From<MItemPtr>> MListPtr<V>{
    pub fn new(map : *mut LinkedMap<MutItem>, list_def : *const ListDefObj, root : *mut RootObject) -> MListPtr<V>{ MListPtr { map, list_def, root, phantom : PhantomData } }

    fn from(&self, item : *mut MutItem) -> V{
        V::from(MItemPtr::new(item, self.list_def, self.root))
    }

    pub fn first(&mut self) -> Option<V> {
        let map = unsafe{ &mut *self.map };
        map.first_mut().map(|r| self.from(r))
    }
    /// Gets a mutable pointer from '&self'
    /// Actually this doesn't violate anything because 'self' only has pointers.
    /// This 'unsafe' is just a decoration
    pub unsafe fn first_const(&self) -> Option<V> {
        let map = &mut *self.map;
        map.first_mut().map(|r| self.from(r))
    }
    pub fn first_id(&self) -> Option<u64> {
        let map = unsafe{ &mut *self.map };
        map.first_id()
    }
    pub fn last(&mut self) -> Option<V> {
        let map = unsafe{ &mut *self.map };
        map.last_mut().map(|r| self.from(r))
    }

    /// gets a mutable pointer from '&self'
    /// actually this doesn't violate anything because 'self' only has pointers.
    /// This 'unsafe' is just a decoration
    pub unsafe fn last_const(&self) -> Option<V> {
        let map = &mut *self.map;
        map.last_mut().map(|r| self.from(r))
    }

    pub fn last_id(&self) -> Option<u64> {
        let map = unsafe{ &mut *self.map };
        map.last_id()
    }
    pub fn get_item(&mut self, id : u64) -> Option<V>{
        let map = unsafe{ &mut *self.map };
        map.get_item_mut(id).map(|b| self.from(b))
    }

    /// gets a mutable pointer from '&self'
    /// actually this doesn't violate anything because 'self' only has pointers.
    /// This 'unsafe' is just a decoration
    pub unsafe fn get_item_const(&self, id : u64) -> Option<V>{
        let map = &mut *self.map;
        map.get_item_mut(id).map(|b| self.from(b))
    }

    pub fn next_id(&self) -> u64{
        let map = unsafe{ &mut *self.map };
        map.next_id()
    }

    pub fn contains_key(&self, key : u64) -> bool{
        let map = unsafe{ &mut *self.map };
        map.contains_key(key)
    }
    pub fn len(&self) -> usize{
        let map = unsafe{ &mut *self.map };
        map.len()
    }
    pub fn is_empty(&self) -> bool {
        let map = unsafe { &mut *self.map };
        map.is_empty()
    }

    pub fn insert(&mut self) -> V{
        self.insert_last()
    }

    pub fn insert_last(&mut self) -> V{
        let map = unsafe{ &mut *self.map };
        let id = map.insert_last(MutItem::default());
        self.get_item(id).unwrap()
    }
    pub fn insert_first(&mut self) -> V{
        let map = unsafe{ &mut *self.map };
        let id = map.insert_first(MutItem::default());
        self.get_item(id).unwrap()
    }

    /// Anything can happen when a removed item is accessed, so be careful
    pub unsafe fn remove(&mut self, id : u64) -> bool {
        let map = &mut *self.map;
        map.remove(id)
    }
    /// Anything can happen when a removed item is accessed, so be careful
    pub unsafe fn remove_first(&mut self) -> bool{
        let map = &mut *self.map;
        map.remove_first()
    }
    /// Anything can happen when a removed item is accessed, so be careful
    pub unsafe fn remove_last(&mut self) -> bool{
        let map =  &mut *self.map;
        map.remove_last()
    }

    pub fn move_to_first(&mut self, id : u64) -> bool {
        let map = unsafe{ &mut *self.map };
        map.move_to_first(id)
    }

    pub fn move_to_last(&mut self, id : u64) -> bool {
        let map = unsafe{ &mut *self.map };
        map.move_to_last(id)
    }

    pub fn move_to_prev(&mut self, next_items_id : u64, id : u64) -> bool{
        let map = unsafe{ &mut *self.map };
        map.move_to_prev(next_items_id, id)
    }

    pub fn move_to_next(&mut self, prev_items_id : u64, id : u64) -> bool{
        let map = unsafe{ &mut *self.map };
        map.move_to_next(prev_items_id, id)
    }

    pub fn iter(&mut self) -> MListPtrIter<V> {
        let map = unsafe { &mut *self.map };
        MListPtrIter::new(unsafe { map.iter_unsafe() }, self.list_def, self.root)
    }

    pub fn iter_from_last(&mut self) -> MListPtrIter<V> {
        let map = unsafe{ &mut *self.map };
        MListPtrIter::new(unsafe{ map.iter_from_last_unsafe() }, self.list_def, self.root)
    }

    pub fn iter_from_id(&mut self, id : u64) -> Option<MListPtrIter<V>> {
        let map = unsafe{ &mut *self.map };
        unsafe { map.iter_from_id_unsafe(id) }.map(|iter| MListPtrIter::new(iter, self.list_def, self.root))
    }
}

pub struct MListPtrIter<V : From<MItemPtr>>{
    iter : LinkedMapUnsafeIter<MutItem>,
    list_def : *const ListDefObj,
    root : *mut RootObject,
    phantom : PhantomData<*mut V>,
}
impl<V : From<MItemPtr>> Iterator for MListPtrIter<V>{
    type Item = (u64, V);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next_mut().map(|(k,v)| (*k, V::from(MItemPtr::new(v, self.list_def, self.root))))
    }
}
impl<V : From<MItemPtr>> MListPtrIter<V>{
    pub(crate) fn new(iter : LinkedMapUnsafeIter<MutItem>, list_def : *const ListDefObj, root : *mut RootObject) -> MListPtrIter<V>{
        MListPtrIter { iter, list_def, root, phantom : PhantomData }
    }

    fn from(&self, item : *mut MutItem) -> V{
        V::from(MItemPtr::new(item, self.list_def, self.root))
    }
    ///現在のカーソルにあるアイテムを返し、カーソルを進める
    pub fn next(&mut self) -> Option<(u64, V)> {
        self.iter.next_mut().map(|(k,v)| (*k, self.from(v)))
    }

    ///前に戻ることが出来る。そして元あった場所を削除し、それによって削除されたアイテムの次にあったアイテムが現在のカーソルの次にくるので、
    /// next2回でそれをとることも出来る。
    ///今ある場所をremoveしたらポインタが不正になって安全にnext/prevできない
    pub fn prev(&mut self) -> Option<(u64, V)> {
        self.iter.prev_mut().map(|(k,v)| (*k, self.from(v)))
    }
    
    pub fn current(&mut self) -> Option<(u64, V)> {
        self.iter.current_mut().map(|(k,v)| (*k,self.from(v)))
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
