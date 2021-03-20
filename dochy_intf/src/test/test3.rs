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
//             let mut map : HashMap<String, Qv<String>> = HashMap::new();
//             map.insert("param1".to_string(), Qv::Val("value_h".to_string()));
//             Item{ map }
//         }
//         pub(crate ) fn param1(&self) -> &Qv<String>{
//             self.map.get("param1").unwrap()
//         }
//         pub(crate ) fn param1_set(&mut self, val : Qv<String>){
//             self.map.insert("param1".to_string(), val);
//         }
//     }
//
//
//
//     #[test]
//     fn it_works_magic() {
//         // let mut item = Item::new();
//         // let val = item.param1();
//         // item.param1_set(Qv::Val("hoge".to_string()));
//         // println!("{:?}", val);
//
//     }
//
//
// }