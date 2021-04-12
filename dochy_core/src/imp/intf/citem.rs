use crate::imp::structs::rust_list::{ConstItem};
use crate::imp::structs::list_value::{ListDefValue, ListSabValue};
//use crate::imp::intf::inner_data::InnerDataPtr;
use crate::imp::structs::rust_param::RustParam;
use crate::imp::structs::qv::Qv;
use crate::imp::structs::list_def_obj::ListDefObj;
use crate::imp::structs::root_obj::RootObject;
use crate::imp::intf::RootObjectPtr;
use crate::imp::intf::clist::CListPtr;
use crate::imp::structs::ref_value::RefSabValue;
use crate::HashM;

#[repr(C)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct CItemPtr {
    item : *const ConstItem,
    list_def : *const ListDefObj,
    root : *mut RootObject,
}

impl CItemPtr {
    pub fn new(item : *const ConstItem, list_def : *const ListDefObj, root : *mut RootObject) -> CItemPtr { CItemPtr { item, list_def, root }}
    pub fn item(&self) -> *const ConstItem { self.item }
    pub fn list_def(&self) -> *const ListDefObj{ self.list_def }
}

pub fn get_cil<T : From<CItemPtr>>(ps : CItemPtr, name : &str) -> Option<CListPtr<T>>{
    let (item, list_def) = unsafe{ (&*ps.item, &*ps.list_def) };
    if let Some(ListDefValue::CilDef(def)) = list_def.default().get(name){
        if let Some(ListSabValue::Cil(data)) = item.values().get(name){
            return Some(CListPtr::new(data.list(), def, ps.root))
        }
    }
    None
}

pub fn get_bool(ps : CItemPtr, name : &str) -> Option<Qv<bool>>{
    if let Some(RustParam::Bool(b)) = get_param(ps, name){
        Some(b.clone())
    } else{ None }
}

pub fn get_bool_def(ps : CItemPtr, name : &str) -> Option<Qv<bool>>{
    if let Some(RustParam::Bool(b)) = get_param_def(ps, name){
        Some(b.clone())
    } else{ None }
}

pub fn get_float(ps : CItemPtr, name : &str) -> Option<Qv<f64>>{
    if let Some(RustParam::Float(b)) = get_param(ps, name){
        Some(b.clone())
    } else{ None }
}

pub fn get_float_def(ps : CItemPtr, name : &str) -> Option<Qv<f64>>{
    if let Some(RustParam::Float(b)) = get_param_def(ps, name){
        Some(b.clone())
    } else{ None }
}

pub fn get_int(ps : CItemPtr, name : &str) -> Option<Qv<i64>>{
    if let Some(RustParam::Int(b)) = get_param(ps, name){
        Some(b.clone())
    } else{ None }
}

pub fn get_int_def(ps : CItemPtr, name : &str) -> Option<Qv<i64>>{
    if let Some(RustParam::Int(b)) = get_param_def(ps, name){
        Some(b.clone())
    } else{ None }
}

pub fn get_str(ps : CItemPtr, name : &str) -> Option<Qv<String>>{
    if let Some(RustParam::String(b)) = get_param(ps, name){
        Some(b.map(|s| s.str().to_string()))
    } else{ None }
}

pub fn get_str_def(ps : CItemPtr, name : &str) -> Option<Qv<String>>{
    if let Some(RustParam::String(b)) = get_param_def(ps, name){
        Some(b.map(|s| s.str().to_string()))
    } else{ None }
}

pub fn get_int_array(ps : CItemPtr, name : &str) -> Option<Qv<Vec<i64>>>{
    if let Some(RustParam::IntArray(b)) = get_param(ps, name){
        Some(b.map(|s| s.vec().clone()))
    } else{
        None
    }
}

pub fn get_int_array_def(ps : CItemPtr, name : &str) -> Option<Qv<Vec<i64>>>{
    if let Some(RustParam::IntArray(b)) = get_param_def(ps, name){
        Some(b.map(|s| s.vec().clone()))
    } else{ None }
}

pub fn get_float_array(ps : CItemPtr, name : &str) -> Option<Qv<Vec<f64>>>{
    if let Some(RustParam::FloatArray(b)) = get_param(ps, name){
        Some(b.map(|s| s.vec().clone()))
    } else{
        None
    }
}

pub fn get_float_array_def(ps : CItemPtr, name : &str) -> Option<Qv<Vec<f64>>>{
    if let Some(RustParam::FloatArray(b)) = get_param_def(ps, name){
        Some(b.map(|s| s.vec().clone()))
    } else{ None }
}

pub fn get_binary(ps : CItemPtr, name : &str) -> Option<Qv<Vec<u8>>>{
    if let Some(RustParam::Binary(b)) = get_param(ps, name){
        Some(b.map(|s| s.vec().clone()))
    } else{
        None
    }
}

pub fn get_binary_def(ps : CItemPtr, name : &str) -> Option<Qv<Vec<u8>>>{
    if let Some(RustParam::Binary(b)) = get_param_def(ps, name){
        Some(b.map(|s| s.vec().clone()))
    } else{ None }
}

pub fn get_immutable_binary<'a, 'b>(ps : CItemPtr, name : &'a str) -> Option<Qv<&'b Vec<u8>>>{
    if let Some(RustParam::Binary(b)) = get_param(ps, name){
        match b{
            Qv::Val(v) => Some(Qv::Val(v.vec())),
            Qv::Null => Some(Qv::Null),
            Qv::Undefined => Some(Qv::Undefined)
        }
    } else{
        None
    }
}


pub fn get_param<'a>(p : CItemPtr, name : &str) -> Option<&'a RustParam>{
    let (item, def) = unsafe{ (&*p.item, &*p.list_def) };
    if let Some(ListSabValue::Param(p)) = item.values().get(name){
        Some(p)
    } else if let Some(ListDefValue::Param(p, _)) = def.default().get(name){
        Some(p)
    } else{
        None
    }
}

pub fn get_param_def<'a, 'b>(def : CItemPtr, name : &'a str) -> Option<&'b RustParam>{
    let def = unsafe{ &*def.list_def };
    if let Some(ListDefValue::Param(p, _)) = def.default().get(name){
        Some(p)
    } else{
        None
    }
}

pub fn get_ref(ps : CItemPtr, list_name : &str) -> Option<Qv<CItemPtr>>{
    let qv = get_ref_id(ps, list_name)?;
    qv.opt_map(|id|{
        let data = super::root::get_table(RootObjectPtr::new(ps.root), list_name).unwrap();
        super::table::get_value(data, id)
    })
}

pub fn get_ref_id(ps : CItemPtr, list_name : &str) -> Option<Qv<String>>{
    let (item, list_def) = unsafe{ (&*ps.item, &*ps.list_def) };
    get_ref_id_impl(item.refs(), list_def, list_name)
}

pub fn get_ref_id_impl(refs : &HashM<String, RefSabValue>, list_def : &ListDefObj, list_name : &str) -> Option<Qv<String>>{
    let qv = if let Some(sab) = refs.get(list_name){
        sab.value()
    } else{
        if let Some(d) = list_def.refs().refs().get(list_name){
            d.value()
        } else{ return None; }
    };
    return Some(qv.clone());
}

pub fn get_enum(ps : CItemPtr) -> Option<(String, String)>{
    let item = unsafe{ &*ps.item };
    get_enum_impl(item.refs())
}

pub fn get_enum_impl(h : &HashM<String, RefSabValue>) -> Option<(String, String)>{
    for (key, value) in h{
        if let Qv::Val(v) = value.value(){
            return Some((key.to_string(), v.to_string()))
        }
    }
    return None;
}