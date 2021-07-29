// use crate::imp::structs::rust_list::MutItem;
// //use crate::imp::intf::inner_data::InnerDataPtr;
// use crate::imp::structs::list_value::{ListDefValue};
// use crate::imp::structs::qv::{Qv};
// use crate::imp::structs::rust_param::RustParam;
// use crate::imp::structs::list_def_obj::ListDefObj;
// use crate::imp::intf::mlist_ptr::MListPtr;
// use crate::imp::intf::{CItemPtr};
// use crate::imp::structs::ref_value::RefSabValue;
// use crate::imp::intf::citem::{get_enum_impl, get_ref_id_impl};
// use crate::imp::structs::rust_string::RustString;
// use crate::imp::structs::rust_array::{RustIntArray, RustFloatArray};
// use crate::structs::RustBinary;
// use crate::imp::structs::list_sab_value::ListSabValue;
// use crate::imp::structs::root_def_obj::RootDefObj;
// use crate::imp::intf::mlist_cptr::MListCPtr;
//
// #[repr(C)]
// #[derive(Debug, PartialEq, Clone, Copy)]
// pub struct MItemCPtr {
//     item : *const MutItem,
//     list_def : *const ListDefObj,
//     root_def: *const RootDefObj,
// }
//
// impl MItemCPtr {
//     ///getだけなら &MutListItemからのポインタでもOKである。その場合setするとundefined behaviorなので、&mut からのポインタを得る必要がある
//     pub fn new(item : *const MutItem, list_def : *const ListDefObj, root_def : *const RootDefObj) -> MItemCPtr {
//         MItemCPtr { item, list_def, root_def }
//     }
//     pub fn item(&self) -> *const MutItem { self.item }
//     pub fn list_def(&self) -> *const ListDefObj{ self.list_def }
// }
//
// pub fn get_mil<T : From<MItemCPtr>>(ps : MItemCPtr, name : &str) -> Option<Option<MListCPtr<T>>> {
//     let (item, list_def) = unsafe { (&*ps.item, &*ps.list_def) };
//     if let Some(ListDefValue::MilDef(md)) = list_def.default().get(name) {
//         if let Some(ListSabValue::Mil(data)) = item.values().get(name) {
//             if let Some(inner) = data {
//                 return Some(Some(MListCPtr::new(inner.list(), md.default(), ps.root_def)))
//             } else {
//                 return Some(None)
//             }
//         }
//     }
//     return None
// }
//
// pub fn get_bool(ps : MItemCPtr, name : &str) -> Option<Qv<bool>>{
//     if let Some(RustParam::Bool(b)) = get_param(ps, name){
//         Some(b.clone())
//     } else{ None }
// }
//
// pub fn get_bool_def(ps : MItemCPtr, name : &str) -> Option<Qv<bool>>{
//     if let Some(RustParam::Bool(b)) = get_param_def(ps, name){
//         Some(b.clone())
//     } else{ None }
// }
//
// pub fn get_int(ps : MItemCPtr, name : &str) -> Option<Qv<i64>>{
//     if let Some(RustParam::Int(b)) = get_param(ps, name){
//         Some(b.clone())
//     } else{ None }
// }
//
// pub fn get_int_def(ps : MItemCPtr, name : &str) -> Option<Qv<i64>>{
//     if let Some(RustParam::Int(b)) = get_param_def(ps, name){
//         Some(b.clone())
//     } else{ None }
// }
//
// pub fn get_float(ps : MItemCPtr, name : &str) -> Option<Qv<f64>>{
//     if let Some(RustParam::Float(b)) = get_param(ps, name){
//         Some(b.clone())
//     } else{ None }
// }
//
// pub fn get_float_def(ps : MItemCPtr, name : &str) -> Option<Qv<f64>>{
//     if let Some(RustParam::Float(b)) = get_param_def(ps, name){
//         Some(b.clone())
//     } else{ None }
// }
//
// pub fn get_str(ps : MItemCPtr, name : &str) -> Option<Qv<String>>{
//     if let Some(RustParam::String(b)) = get_param(ps, name){
//         Some(b.map(|s| s.str().to_string()))
//     } else{ None }
// }
//
// pub fn get_str_def(ps : MItemCPtr, name : &str) -> Option<Qv<String>>{
//     if let Some(RustParam::String(b)) = get_param_def(ps, name){
//         Some(b.map(|s| s.str().to_string()))
//     } else{ None }
// }
//
// pub fn get_int_array(ps : MItemCPtr, name : &str) -> Option<Qv<Vec<i64>>>{
//     if let Some(RustParam::IntArray(b)) = get_param(ps, name){
//         Some(b.map(|s| s.vec().clone()))
//     } else{
//         None
//     }
// }
//
// pub fn get_int_array_def(ps : MItemCPtr, name : &str) -> Option<Qv<Vec<i64>>>{
//     if let Some(RustParam::IntArray(b)) = get_param_def(ps, name){
//         Some(b.map(|s| s.vec().clone()))
//     } else{
//         None
//     }
// }
// pub fn get_float_array(ps : MItemCPtr, name : &str) -> Option<Qv<Vec<f64>>>{
//     if let Some(RustParam::FloatArray(b)) = get_param(ps, name){
//         Some(b.map(|s| s.vec().clone()))
//     } else{
//         None
//     }
// }
// pub fn get_float_array_def(ps : MItemCPtr, name : &str) -> Option<Qv<Vec<f64>>>{
//     if let Some(RustParam::FloatArray(b)) = get_param_def(ps, name){
//         Some(b.map(|s| s.vec().clone()))
//     } else{
//         None
//     }
// }
// pub fn get_binary(ps : MItemCPtr, name : &str) -> Option<Qv<Vec<u8>>>{
//     if let Some(RustParam::Binary(b)) = get_param(ps, name){
//         Some(b.map(|s| s.vec().clone()))
//     } else{
//         None
//     }
// }
// pub fn get_binary_def(ps : MItemCPtr, name : &str) -> Option<Qv<Vec<u8>>>{
//     if let Some(RustParam::Binary(b)) = get_param_def(ps, name){
//         Some(b.map(|s| s.vec().clone()))
//     } else{
//         None
//     }
// }
//
// pub fn get_immutable_binary<'a, 'b>(ps : MItemCPtr, name : &'a str) -> Option<Qv<&'b Vec<u8>>>{
//     if let Some(RustParam::Binary(b)) = get_param(ps, name){
//
//         match b{
//              Qv::Val(v) => Some(Qv::Val(v.vec())),
//              Qv::Null => Some(Qv::Null),
//              Qv::Undefined => Some(Qv::Undefined)
//         }
//     } else{
//         None
//     }
// }
//
// pub fn get_immutable_str<'a, 'b>(ps : MItemCPtr, name : &'a str) -> Option<Qv<&'b String>>{
//     if let Some(RustParam::String(b)) = get_param(ps, name){
//         match b{
//             Qv::Val(v) => Some(Qv::Val(v.string())),
//             Qv::Null => Some(Qv::Null),
//             Qv::Undefined => Some(Qv::Undefined)
//         }
//     } else{
//         None
//     }
// }
// pub fn get_immutable_int_array<'a, 'b>(ps : MItemCPtr, name : &'a str) -> Option<Qv<&'b Vec<i64>>>{
//     if let Some(RustParam::IntArray(b)) = get_param(ps, name){
//         match b{
//             Qv::Val(v) => Some(Qv::Val(v.vec())),
//             Qv::Null => Some(Qv::Null),
//             Qv::Undefined => Some(Qv::Undefined)
//         }
//     } else{
//         None
//     }
// }
// pub fn get_immutable_float_array<'a, 'b>(ps : MItemCPtr, name : &'a str) -> Option<Qv<&'b Vec<f64>>>{
//     if let Some(RustParam::FloatArray(b)) = get_param(ps, name){
//         match b{
//             Qv::Val(v) => Some(Qv::Val(v.vec())),
//             Qv::Null => Some(Qv::Null),
//             Qv::Undefined => Some(Qv::Undefined)
//         }
//     } else{
//         None
//     }
// }
//
//
//
// pub fn get_param<'a>(ps : MItemCPtr, name : &str) -> Option<&'a RustParam>{
//     let (item, def) = unsafe{ (&*ps.item, &*ps.list_def) };
//     if let Some(ListSabValue::Param(p)) = item.values().get(name){
//         Some(p)
//     } else if let Some(ListDefValue::Param(p, _)) = def.default().get(name){
//         Some(p)
//     } else{
//         None
//     }
// }
//
// pub fn get_param_def<'a>(ps : MItemCPtr, name : &str) -> Option<&'a RustParam>{
//     let def = unsafe{ &*ps.list_def };
//     if let Some(ListDefValue::Param(p, _)) = def.default().get(name){
//         Some(p)
//     } else{
//         None
//     }
// }
//
//
// pub fn get_ref(ps : MItemCPtr, list_name : &str) -> Option<Qv<CItemPtr>>{
//     let qv = get_ref_id(ps, list_name)?;
//     qv.opt_map(|id|{
//         let data = super::root::get_table(ps.root_def, list_name).unwrap();
//         super::table::get_value(data, id)
//     })
// }
//
// pub fn get_ref_id(ps : MItemCPtr, list_name : &str) -> Option<Qv<String>>{
//     let (item, list_def) = unsafe{ (ps.item.as_ref().unwrap(), ps.list_def.as_ref().unwrap()) };
//     get_ref_id_impl(item.refs(), list_def, list_name)
// }
//
// pub fn get_enum(ps : MItemCPtr) -> Option<(String, String)>{
//     let item = unsafe{ ps.item.as_ref().unwrap() };
//     get_enum_impl(item.refs())
// }
//
// /// Checks if the param hasn't been modified yet
// pub fn is_unmodified(ps : MItemCPtr, name : &str) -> bool{
//     let item = unsafe { &*ps.item };
//     !item.values().contains_key(name)
// }