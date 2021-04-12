use crate::imp::structs::root_obj::RootObject;
use crate::imp::structs::qv::Qv;
use crate::HashM;
use crate::imp::structs::root_value::RootValue;
use crate::imp::structs::rust_param::RustParam;
use crate::imp::structs::rust_string::RustString;
use crate::imp::intf::clist::CListPtr;
use crate::imp::intf::mlist::MListPtr;
use crate::imp::intf::table::TablePtr;
use crate::imp::intf::mitem::MItemPtr;
use crate::imp::intf::citem::CItemPtr;
use crate::imp::structs::rust_array::{RustIntArray, RustFloatArray};
use crate::structs::RustBinary;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct RootObjectPtr{
    ptr : *mut RootObject
}
impl RootObjectPtr {
    pub fn new(ptr: *mut RootObject) -> RootObjectPtr { RootObjectPtr { ptr } }
}

pub fn get_bool(root : RootObjectPtr, name : &str) -> Option<Qv<bool>>{
    if let Some(RustParam::Bool(b)) = get_param(root, name){
        Some(b.clone())
    } else{
        None
    }
}
pub fn get_bool_def(root : RootObjectPtr, name : &str) -> Option<Qv<bool>>{
    if let Some(RustParam::Bool(b)) = get_param_def(root, name){
        Some(b.clone())
    } else{
        None
    }
}

pub fn get_float(root : RootObjectPtr, name : &str) -> Option<Qv<f64>>{
    if let Some(RustParam::Float(b)) = get_param(root, name){
        Some(b.clone())
    } else{
        None
    }
}

pub fn get_float_def(root : RootObjectPtr, name : &str) -> Option<Qv<f64>>{
    if let Some(RustParam::Float(b)) = get_param_def(root, name){
        Some(b.clone())
    } else{
        None
    }
}

pub fn get_int(root : RootObjectPtr, name : &str) -> Option<Qv<i64>>{
    if let Some(RustParam::Int(b)) = get_param(root, name){
        Some(b.clone())
    } else{
        None
    }
}

pub fn get_int_def(root : RootObjectPtr, name : &str) -> Option<Qv<i64>>{
    if let Some(RustParam::Int(b)) = get_param_def(root, name){
        Some(b.clone())
    } else{
        None
    }
}

pub fn get_str(root : RootObjectPtr, name : &str) -> Option<Qv<String>>{
    if let Some(RustParam::String(b)) = get_param(root, name){
        Some(b.map(|s| s.str().to_string()))
    } else{
        None
    }
}

pub fn get_str_def(root : RootObjectPtr, name : &str) -> Option<Qv<String>>{
    if let Some(RustParam::String(b)) = get_param_def(root, name){
        Some(b.map(|s| s.str().to_string()))
    } else{
        None
    }
}

pub fn get_int_array(root : RootObjectPtr, name : &str) -> Option<Qv<Vec<i64>>>{
    if let Some(RustParam::IntArray(b)) = get_param(root, name){
        Some(b.map(|s| s.vec().clone()))
    } else{
        None
    }
}

pub fn get_int_array_def(root : RootObjectPtr, name : &str) -> Option<Qv<Vec<i64>>>{
    if let Some(RustParam::IntArray(b)) = get_param_def(root, name){
        Some(b.map(|s| s.vec().clone()))
    } else{
        None
    }
}

pub fn get_float_array(root : RootObjectPtr, name : &str) -> Option<Qv<Vec<f64>>>{
    if let Some(RustParam::FloatArray(b)) = get_param(root, name){
        Some(b.map(|s| s.vec().clone()))
    } else{
        None
    }
}

pub fn get_float_array_def(root : RootObjectPtr, name : &str) -> Option<Qv<Vec<f64>>>{
    if let Some(RustParam::FloatArray(b)) = get_param_def(root, name){
        Some(b.map(|s| s.vec().clone()))
    } else{
        None
    }
}
pub fn get_binary(root : RootObjectPtr, name : &str) -> Option<Qv<Vec<u8>>>{
    if let Some(RustParam::Binary(b)) = get_param(root, name){
        Some(b.map(|s| s.vec().clone()))
    } else{
        None
    }
}
pub fn get_binary_def(root : RootObjectPtr, name : &str) -> Option<Qv<Vec<u8>>>{
    if let Some(RustParam::Binary(b)) = get_param_def(root, name){
        Some(b.map(|s| s.vec().clone()))
    } else{
        None
    }
}
pub fn get_immutable_binary(root : RootObjectPtr, name : &str) -> Option<Qv<&Vec<u8>>>{
    if let Some(RustParam::Binary(b)) = get_param(root, name){
        match b{
            Qv::Val(v) => Some(Qv::Val(v.vec())),
            Qv::Null => Some(Qv::Null),
            Qv::Undefined => Some(Qv::Undefined)
        }
    } else{
        None
    }
}
pub fn get_mutable_binary<'a, 'b>(ps : RootObjectPtr, name : &'a str) -> Option<Qv<&'b mut Vec<u8>>>{
    let item =  unsafe{ &mut *ps.ptr };
    if let Some(RustParam::Binary(b)) = get_param_mut(item.sabun_mut(), name){
        match b{
            Qv::Val(v) => Some(Qv::Val(v.vec_mut())),
            Qv::Null => Some(Qv::Null),
            Qv::Undefined => Some(Qv::Undefined)
        }
    } else{
        None
    }
}

pub fn get_table(root_ptr : RootObjectPtr, name : &str) -> Option<TablePtr>{
    let root = unsafe{ &*root_ptr.ptr };
    if let Some((_, RootValue::Table(d))) = root.default().get(name){
        Some(TablePtr::new(d, root_ptr.ptr))
    } else{ None }
}


pub fn get_clist<T : From<CItemPtr>>(root_ptr : RootObjectPtr, name : &str) -> Option<CListPtr<T>>{
    let root = unsafe{ &*root_ptr.ptr };
    if let Some((_,RootValue::CList(l))) = root.default().get(name){
        Some(CListPtr::new(l.list(),l.default(), root_ptr.ptr))
    } else{ None }
}

pub fn get_mlist<T : From<MItemPtr>>(root : RootObjectPtr, name : &str) -> Option<MListPtr<T>>{
    let root = unsafe{ &mut *root.ptr };
    if let Some((_,RootValue::MList(l))) = root.default_mut().get_mut(name){
        Some(MListPtr::new(l.list_mut(), l.default(), root))
    } else{ None }
}

pub fn get_param<'a>(ps : RootObjectPtr, name : &str) -> Option<&'a RustParam> {
    let ptr = unsafe{ &*ps.ptr };
    let def = ptr.default();
    let sab = ptr.sabun();

    if let Some(p) = sab.get(name) {
        Some(p)
    } else if let Some((_, RootValue::Param(p, _v))) = def.get(name) {
        Some(p)
    } else { None }
}
pub fn get_param_mut<'a>(sab : &'a mut HashM<String, RustParam>, name : &str) -> Option<&'a mut RustParam> {
    if let Some(p) = sab.get_mut(name) {
        Some(p)
    } else { None }
}
pub fn get_param_def<'a>(ps : RootObjectPtr, name : &str) -> Option<&'a RustParam> {
    let def = unsafe{ &*ps.ptr}.default();

    if let Some((_, RootValue::Param(p, _v))) = def.get(name) {
        Some(p)
    } else { None }
}

pub fn set_sabun(root : RootObjectPtr, name : &str, val : RustParam) -> bool{
    let root = unsafe{ &mut *root.ptr };
    match root.set_sabun(name.to_string(), val){
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn set_bool(root : RootObjectPtr, name : &str, val : Qv<bool>) -> bool{
    set_sabun(root, name, RustParam::Bool(val))
}
pub fn set_float(root : RootObjectPtr, name : &str, val : Qv<f64>) -> bool{
    set_sabun(root, name, RustParam::Float(val))
}
pub fn set_int(root : RootObjectPtr, name : &str, val : Qv<i64>) -> bool{
    set_sabun(root, name, RustParam::Int(val))
}
pub fn set_str(root : RootObjectPtr, name : &str, val : Qv<String>) -> bool{
    set_sabun(root,name, RustParam::String(val.into_map(|s| RustString::new(s))))
}
pub fn set_int_array(root : RootObjectPtr, name : &str, val : Qv<Vec<i64>>) -> bool{
    set_sabun(root,name, RustParam::IntArray(val.into_map(|s| RustIntArray::new(s))))
}
pub fn set_float_array(root : RootObjectPtr, name : &str, val : Qv<Vec<f64>>) -> bool{
    set_sabun(root,name, RustParam::FloatArray(val.into_map(|s| RustFloatArray::new(s))))
}
pub fn set_binary(root : RootObjectPtr, name : &str, val : Qv<Vec<u8>>) -> bool{
    set_sabun(root,name, RustParam::Binary(val.into_map(|s| RustBinary::new(s))))
}