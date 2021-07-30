use crate::imp::structs::rust_list::{ConstItem};
use crate::imp::structs::list_value::{ListDefValue};
//use crate::imp::intf::inner_data::InnerDataPtr;
use crate::imp::structs::rust_param::RustParam;
use crate::imp::structs::qv::Qv;
use crate::imp::structs::list_def_obj::ListDefObj;
use crate::imp::intf::clist::CListPtr;
use crate::imp::structs::ref_value::RefSabValue;
use crate::HashM;
use crate::imp::structs::list_sab_value::ListSabValue;
use crate::imp::structs::root_def_obj::RootDefObj;

#[repr(C)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct CItemPtr<'a> {
    item : &'a ConstItem,
    list_def : &'a ListDefObj,
    root_def : &'a RootDefObj,
}

impl<'a> CItemPtr<'a> {
    pub fn new<'b>(item : &'b ConstItem, list_def : &'b ListDefObj, root_def : &'b RootDefObj) -> CItemPtr<'b> { CItemPtr { item, list_def, root_def }}
    pub fn item(&self) -> &'a ConstItem { self.item }
    pub fn list_def(&self) -> &'a ListDefObj{ self.list_def }
}

pub fn get_cil<'a, T : From<CItemPtr<'a>>>(ps : CItemPtr, name : &str) -> Option<CListPtr<'a, T>>{
    let (item, list_def) = (ps.item, ps.list_def);
    if let Some(ListDefValue::CilDef(def)) = list_def.default().get(name){
        if let Some(ListSabValue::Cil(data)) = item.values().get(name){
            return Some(CListPtr::new(data.list(), def, ps.root_def))
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
pub fn get_immutable_str<'a, 'b>(ps : CItemPtr, name : &'a str) -> Option<Qv<&'b String>>{
    if let Some(RustParam::String(b)) = get_param(ps, name){
        match b{
            Qv::Val(v) => Some(Qv::Val(v.string())),
            Qv::Null => Some(Qv::Null),
            Qv::Undefined => Some(Qv::Undefined)
        }
    } else{
        None
    }
}
pub fn get_immutable_int_array<'a, 'b>(ps : CItemPtr, name : &'a str) -> Option<Qv<&'b Vec<i64>>>{
    if let Some(RustParam::IntArray(b)) = get_param(ps, name){
        match b{
            Qv::Val(v) => Some(Qv::Val(v.vec())),
            Qv::Null => Some(Qv::Null),
            Qv::Undefined => Some(Qv::Undefined)
        }
    } else{
        None
    }
}
pub fn get_immutable_float_array<'a, 'b>(ps : CItemPtr, name : &'a str) -> Option<Qv<&'b Vec<f64>>>{
    if let Some(RustParam::FloatArray(b)) = get_param(ps, name){
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
    let (item, def) = (p.item, p.list_def);
    if let Some(ListSabValue::Param(p)) = item.values().get(name){
        Some(p)
    } else if let Some(ListDefValue::Param(p, _)) = def.default().get(name){
        Some(p)
    } else{
        None
    }
}

pub fn get_param_def<'a, 'b>(def : CItemPtr, name : &'a str) -> Option<&'b RustParam>{
    let def = def.list_def;
    if let Some(ListDefValue::Param(p, _)) = def.default().get(name){
        Some(p)
    } else{
        None
    }
}

pub fn get_ref<'a>(ps : CItemPtr<'a>, list_name : &str) -> Option<Qv<CItemPtr<'a>>>{
    let qv = get_ref_id(ps, list_name)?;
    qv.opt_map(|id|{
        let data = super::root::get_table(ps.root_def, list_name).unwrap();
        super::table::get_value(data, id)
    })
}

pub fn get_ref_id(ps : CItemPtr, list_name : &str) -> Option<Qv<String>>{
    let (item, list_def) = (ps.item, ps.list_def);
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
    let item = ps.item;
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