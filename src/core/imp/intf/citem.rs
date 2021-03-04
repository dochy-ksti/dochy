use crate::core::imp::structs::rust_list::{ConstItem};
use crate::core::imp::structs::list_value::{ListDefValue, ListSabValue};
//use crate::core::imp::intf::inner_data::InnerDataPtr;
use crate::core::imp::structs::rust_param::RustParam;
use crate::core::imp::structs::qv::Qv;
use crate::core::imp::structs::list_def_obj::ListDefObj;
use crate::core::imp::structs::root_obj::RootObject;
use crate::core::imp::intf::RootObjectPtr;
use crate::core::imp::intf::clist::CListPtr;
use crate::core::imp::structs::ref_value::RefSabValue;
use crate::core::HashM;

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
    let (item,list_def) = unsafe{ (&*ps.item, &*ps.list_def) };
    if let Some(RustParam::Bool(b)) = get_param(item, list_def, name){
        Some(b.clone())
    } else{ None }
}

pub fn get_float(ps : CItemPtr, name : &str) -> Option<Qv<f64>>{
    let (item,list_def) = unsafe{ (&*ps.item, &*ps.list_def) };
    if let Some(RustParam::Float(b)) = get_param(item, list_def, name){
        Some(b.clone())
    } else{ None }
}

pub fn get_int(ps : CItemPtr, name : &str) -> Option<Qv<i64>>{
    let (item,list_def) = unsafe{ (&*ps.item, &*ps.list_def) };
    if let Some(RustParam::Int(b)) = get_param(item, list_def, name){
        Some(b.clone())
    } else{ None }
}

pub fn get_str(ps : CItemPtr, name : &str) -> Option<Qv<String>>{
    let (item,list_def) = unsafe{ (&*ps.item, &*ps.list_def) };
    if let Some(RustParam::String(b)) = get_param(item, list_def, name){
        Some(b.map(|s| s.str().to_string()))
    } else{ None }
}

pub fn get_int_array(ps : CItemPtr, name : &str) -> Option<Qv<Vec<i64>>>{
    let (item,list_def) = unsafe{ (&*ps.item, &*ps.list_def) };
    if let Some(RustParam::IntArray(b)) = get_param(item, list_def, name){
        Some(b.map(|s| s.vec().clone()))
    } else{
        None
    }
}

pub fn get_float_array(ps : CItemPtr, name : &str) -> Option<Qv<Vec<f64>>>{
    let (item,list_def) = unsafe{ (&*ps.item, &*ps.list_def) };
    if let Some(RustParam::FloatArray(b)) = get_param(item, list_def, name){
        Some(b.map(|s| s.vec().clone()))
    } else{
        None
    }
}

pub fn get_param<'a>(item : &'a ConstItem, def : &'a ListDefObj, name : &str) -> Option<&'a RustParam>{
    if let Some(ListSabValue::Param(p)) = item.values().get(name){
        Some(p)
    } else if let Some(ListDefValue::Param(p, _)) = def.default().get(name){
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
    get_ref_id_imol(item.refs(), list_def, list_name)
}

pub fn get_ref_id_imol(refs : &HashM<String, RefSabValue>, list_def : &ListDefObj, list_name : &str) -> Option<Qv<String>>{
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