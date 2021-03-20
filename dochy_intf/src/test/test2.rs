// #[cfg(test)]
// #[allow(dead_code)]
// mod tests {
//     use std::collections::HashMap;
//     use nougami_core::intf::{RustStrPtr, GeneralIter};
//     use std::ffi::CStr;
//     use std::os::raw::c_char;
//     use nougami_core::structs::Qv;
//     use nougami_core::intf::null_or::NullOr;
//
//     pub(crate ) struct Item{
//         pub(crate ) map : HashMap<String, Qv<String>> ,
//     }
//     impl Item{
//         pub(crate ) fn new() -> Item{
//             let mut map : HashMap<String, String> = HashMap::new();
//             map.insert("param1".to_string(), "value_h".to_string());
//             Item{ map }
//         }
//         pub(crate ) fn get(&self, key : &str) -> Option<&Qv<String>>{
//             self.map.get(key)
//         }
//     }
//     pub(crate ) struct List{
//         pub(crate ) vec : Vec<Item>
//     }
//     impl List{
//         pub(crate ) fn new() -> List{
//             List{ vec : vec![Item::new()] }
//         }
//         pub(crate ) fn get(&self, index : usize) -> &Item{
//             self.vec.get(index).unwrap()
//         }
//     }
//
//     pub(crate ) struct Root{
//         list : List
//     }
//     impl Root{
//         pub(crate ) fn new() -> Root{
//             Root{ list : List::new() }
//         }
//     }
//
//
//
//     #[repr(C)]
//     pub(crate ) struct RootMagicIntf{
//         pub(crate ) root : Root
//     }
//     impl RootMagicIntf{
//         pub(crate ) fn new(root : Root) -> RootMagicIntf{
//             RootMagicIntf{ root }
//         }
//         pub(crate ) fn list(&self) -> ListMagicIntf{
//             ListMagicIntf::new(&self.root.list)
//         }
//     }
//
//     #[repr(C)] #[derive(Debug, Clone, Copy)]
//     pub(crate ) struct ListMagicIntf{
//         list : *const List,
//     }
//     impl ListMagicIntf{
//         pub(crate ) fn new(list : *const List) -> ListMagicIntf{
//             ListMagicIntf{ list }
//         }
//         pub(crate ) fn get(&self, index : usize) -> ItemMagicIntf{
//             let list = unsafe{ &*self.list };
//             ItemMagicIntf::new(list.get(index))
//         }
//         pub(crate ) fn len(&self) -> usize{
//             let list = unsafe{ &*self.list };
//             list.vec.len()
//         }
//         pub(crate ) fn general_iter(&self) -> GeneralIter<ListMagicIntf, ItemMagicIntf>{
//             GeneralIter::new(self.len(), self.clone(), ListMagicIntf::get)
//         }
//     }
//     #[allow(non_snake_case)] #[no_mangle]
//     pub(crate ) extern "C" fn ListMagicIntf_get(list : ListMagicIntf, index : usize) -> ItemMagicIntf{
//         list.get(index)
//     }
//     #[allow(non_snake_case)] #[no_mangle]
//     pub(crate ) extern "C" fn ListMagicIntf_len(list : ListMagicIntf) -> usize{
//         list.len()
//     }
//
//     #[repr(C)]
//     pub(crate ) struct ItemMagicIntf{
//         item : *const Item,
//     }
//     impl ItemMagicIntf{
//         pub(crate ) fn new(item : *const Item) -> ItemMagicIntf{ ItemMagicIntf{ item }}
//         pub(crate ) fn param1(&self) -> NullOr<String>{
//             let item = unsafe{ &*self.item };
//             let qv = item.get("param1").unwrap().clone();
//             NullOr::from_qv(qv).unwrap()
//         }
//         pub(crate ) fn set_param1(&self, s : NullOr<String>){
//             let item = unsafe{ &mut *(self.item as *mut Item) };
//             item.map.insert("param1".to_string(), s.into_qv());
//         }
//     }
//     #[allow(non_snake_case)]
//     pub(crate ) extern "C" fn ItemMagicIntf_param1(item : *const ItemMagicIntf) -> RustStrPtr{
//         let item = unsafe{ &*(*item).item };
//         RustStrPtr::new(item.get("param1").unwrap())
//     }
//     #[allow(non_snake_case)]
//     pub(crate ) extern "C" fn ItemMagicIntf_set_param1(item : *const ItemMagicIntf, s : *const c_char){
//         let item = unsafe{ &mut *(item as *mut ItemMagicIntf) };
//         let s = unsafe{ CStr::from_ptr(s) };
//         item.set_param1(s.to_str().unwrap().to_string())
//     }
//
//
//
//
//     #[test]
//     fn it_works_magic() {
//
//
//
//         let hoge = {
//             let root = Root::new();
//             let intf = RootMagicIntf::new(root);
//             let list = intf.list();
//             let item = list.get(0);
//             println!("magic param1 {}", item.param1());
//             item.set_param1("set param1".to_string());
//             let p1 = ItemMagicIntf_param1(&item);
//             p1
//
//         };
//         println!("magic param1 {}", hoge.to_string())
//
//     }
//
//
// }