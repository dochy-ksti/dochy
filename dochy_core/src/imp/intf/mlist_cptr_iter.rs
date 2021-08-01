// use crate::imp::structs::linked_m::{LinkedMap};
// use crate::imp::structs::rust_list::MutItem;
// use std::marker::PhantomData;
// use crate::imp::structs::list_def_obj::ListDefObj;
// use crate::imp::intf::mitem_ptr::MItemPtr;
// use crate::imp::structs::root_def_obj::RootDefObj;
// use crate::imp::structs::linked_map_unsafe_citer::LinkedMapUnsafeCIter;
//
//
// #[derive(Debug)]
// pub struct MListCPtrIter<V : From<MItemCPtr>>{
//     iter : LinkedMapUnsafeCIter<MutItem>,
//     list_def : *const ListDefObj,
//     root_def : *const RootDefObj,
//     phantom : PhantomData<*const V>,
// }
// impl<V : From<MItemCPtr>> Iterator for MListCPtrIter<V>{
//     type Item = (u64, V);
//
//     fn next(&mut self) -> Option<Self::Item> {
//         self.iter.next().map(|(k,v)| (*k, V::from(MItemCPtr::new(v, self.list_def, self.root_def))))
//     }
// }
// impl<V : From<MItemCPtr>> MListCPtrIter<V>{
//     pub(crate) fn new(iter : LinkedMapUnsafeCIter<MutItem>, list_def : *const ListDefObj, root_def : *const RootDefObj) -> MListCPtrIter<V>{
//         MListCPtrIter { iter, list_def, root_def, phantom : PhantomData }
//     }
//
//     fn from(&self, item : *const MutItem) -> V{
//         V::from(MItemCPtr::new(item, self.list_def, self.root_def))
//     }
//     ///現在のカーソルにあるアイテムを返し、カーソルを進める
//     pub fn next(&mut self) -> Option<(u64, V)> {
//         self.iter.next().map(|(k,v)| (*k, self.from(v)))
//     }
//
//     ///前に戻ることが出来る。そして元あった場所を削除し、それによって削除されたアイテムの次にあったアイテムが現在のカーソルの次にくるので、
//     /// next2回でそれをとることも出来る。
//     ///今ある場所をremoveしたらポインタが不正になって安全にnext/prevできない
//     pub fn prev(&mut self) -> Option<(u64, V)> {
//         self.iter.prev().map(|(k,v)| (*k, self.from(v)))
//     }
//
//     pub fn current(&mut self) -> Option<(u64, V)> {
//         self.iter.current().map(|(k,v)| (*k,self.from(v)))
//     }
//
//     pub fn is_available(&self) -> bool {
//         self.iter.is_available()
//     }
//
//     pub fn is_first(&self) -> bool {
//         self.iter.is_first()
//     }
//
//     pub fn is_last(&self) -> bool {
//         self.iter.is_last()
//     }
// }
