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

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct RootObjectPtr{
    ptr : *mut RootObject
}
impl RootObjectPtr {
    pub fn new(ptr: *mut RootObject) -> RootObjectPtr { RootObjectPtr { ptr } }
}

pub fn get_bool(root : RootObjectPtr, name : &str) -> Option<Qv<bool>>{
    let root = unsafe{ &*root.ptr };
    if let Some(RustParam::Bool(b)) = get_param(root.default(), root.sabun(), name){
        Some(b.clone())
    } else{
        None
    }
}

pub fn get_float(root : RootObjectPtr, name : &str) -> Option<Qv<f64>>{
    let root = unsafe{ &*root.ptr };
    if let Some(RustParam::Float(b)) = get_param(root.default(), root.sabun(), name){
        Some(b.clone())
    } else{
        None
    }
}

pub fn get_int(root : RootObjectPtr, name : &str) -> Option<Qv<i64>>{
    let root = unsafe{ &*root.ptr };
    if let Some(RustParam::Int(b)) = get_param(root.default(), root.sabun(), name){
        Some(b.clone())
    } else{
        None
    }
}


pub fn get_str(root : RootObjectPtr, name : &str) -> Option<Qv<String>>{
    let root = unsafe{ &*root.ptr };
    if let Some(RustParam::String(b)) = get_param(root.default(), root.sabun(), name){
        Some(b.map(|s| s.str().to_string()))
    } else{
        None
    }
}

pub fn get_int_array(root : RootObjectPtr, name : &str) -> Option<Qv<Vec<i64>>>{
    let root = unsafe{ &*root.ptr };
    if let Some(RustParam::IntArray(b)) = get_param(root.default(), root.sabun(), name){
        Some(b.map(|s| s.vec().clone()))
    } else{
        None
    }
}

pub fn get_float_array(root : RootObjectPtr, name : &str) -> Option<Qv<Vec<f64>>>{
    let root = unsafe{ &*root.ptr };
    if let Some(RustParam::FloatArray(b)) = get_param(root.default(), root.sabun(), name){
        Some(b.map(|s| s.vec().clone()))
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

pub fn get_param<'a>(def : &'a HashM<String, (usize, RootValue)>, sab : &'a HashM<String, RustParam>, name : &str) -> Option<&'a RustParam>{
    if let Some((_,RootValue::Param(p,_v))) = def.get(name){
        if let Some(p) = sab.get(name){
            Some(p)
        } else{
            Some(p)
        }
    } else { None }
}

pub fn set_bool(root : RootObjectPtr, name : &str, val : Qv<bool>) -> bool{
    let root = unsafe{ &mut *root.ptr };
    match root.set_sabun(name.to_string(), RustParam::Bool(val)){
        Ok(_) => true,
        Err(_) => false,
    }
}
pub fn set_float(root : RootObjectPtr, name : &str, val : Qv<f64>) -> bool{
    let root = unsafe{ &mut *root.ptr };
    match root.set_sabun(name.to_string(), RustParam::Float(val)){
        Ok(_) => true,
        Err(_) => false,
    }
}
pub fn set_int(root : RootObjectPtr, name : &str, val : Qv<i64>) -> bool{
    let root = unsafe{ &mut *root.ptr };
    match root.set_sabun(name.to_string(), RustParam::Int(val)){
        Ok(_) => true,
        Err(_) => false,
    }
}
pub fn set_str(root : RootObjectPtr, name : &str, val : Qv<String>) -> bool{
    let root = unsafe{ &mut *root.ptr };
    match root.set_sabun(name.to_string(), RustParam::String(val.into_map(|s| RustString::new(s)))){
        Ok(_) => true,
        Err(_) => false,
    }
}
pub fn set_int_array(root : RootObjectPtr, name : &str, val : Qv<Vec<i64>>) -> bool{
    let root = unsafe{ &mut *root.ptr };
    match root.set_sabun(name.to_string(), RustParam::IntArray(val.into_map(|s| RustIntArray::new(s)))){
        Ok(_) => true,
        Err(_) => false,
    }
}
pub fn set_float_array(root : RootObjectPtr, name : &str, val : Qv<Vec<f64>>) -> bool{
    let root = unsafe{ &mut *root.ptr };
    match root.set_sabun(name.to_string(), RustParam::FloatArray(val.into_map(|s| RustFloatArray::new(s)))){
        Ok(_) => true,
        Err(_) => false,
    }
}