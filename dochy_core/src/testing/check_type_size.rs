// #[cfg(test)]
// mod tests {
//     use crate::imp::structs::rust_value::RustValue;
//     use crate::imp::structs::root_value::RootValue;
//     use crate::imp::structs::list_value::{ListDefValue, ListSabValue};
//     use crate::imp::structs::ref_value::{RefValue, RefSabValue};
//     use crate::imp::structs::rust_param::RustParam;
//     use crate::imp::structs::rust_list::{ConstData, ConstList, MutList, InnerData, InnerList, InnerMutList};
//     use crate::imp::structs::list_def_obj::ListDefObj;
//     use crate::imp::structs::inner_mut_def_obj::InnerMutDefObj;
//
//     #[test]
//     fn check_size() {
//         println!("RustValue {} ", std::mem::size_of::<RustValue>());
//         println!("RootValue {} ", std::mem::size_of::<RootValue>());
//         println!("ListDefValue {} ", std::mem::size_of::<ListDefValue>());
//         println!("ListSabValue {} ", std::mem::size_of::<ListSabValue>());
//         println!("refvalue {} ", std::mem::size_of::<RefValue>());
//         println!("RefSabValue {} ", std::mem::size_of::<RefSabValue>());
//         println!("RustParam {} ", std::mem::size_of::<RustParam>());
//         println!("ConstData {} ", std::mem::size_of::<ConstData>());
//         println!("ConstList {} ", std::mem::size_of::<ConstList>());
//         println!("MutList {} ", std::mem::size_of::<MutList>());
//         println!("InnerData {} ", std::mem::size_of::<InnerData>());
//         println!("InnerList {} ", std::mem::size_of::<InnerList>());
//         println!("opt_InnerMutList {} ", std::mem::size_of::<Option<InnerMutList>>());
//         println!("ListDefObj {} ", std::mem::size_of::<ListDefObj>());
//         println!("InnerMutDefObj {} ", std::mem::size_of::<InnerMutDefObj>());
//     }
// }