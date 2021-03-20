use crate::imp::structs::rust_list::MutItem;
//use crate::imp::intf::inner_data::InnerDataPtr;
use crate::imp::structs::list_value::{ListDefValue, ListSabValue};
use crate::imp::structs::qv::Qv;
use crate::imp::structs::rust_param::RustParam;
use crate::imp::structs::list_def_obj::ListDefObj;
use crate::imp::structs::root_obj::RootObject;
use crate::imp::intf::mlist::MListPtr;
use crate::imp::intf::{CItemPtr, RootObjectPtr};
use crate::imp::structs::ref_value::RefSabValue;
use crate::imp::intf::citem::{get_enum_impl, get_ref_id_imol};
use crate::imp::structs::util::set_sabun::SetSabunError;
use crate::imp::structs::rust_string::RustString;
use crate::imp::structs::rust_array::{RustIntArray, RustFloatArray};

#[repr(C)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct MItemPtr {
    item : *mut MutItem,
    list_def : *const ListDefObj,
    root : *mut RootObject,
}

impl MItemPtr {
    ///getだけなら &MutListItemからのポインタでもOKである。その場合setするとundefined behaviorなので、&mut からのポインタを得る必要がある
    pub fn new(item : *mut MutItem, list_def : *const ListDefObj, root : *mut RootObject) -> MItemPtr {
        MItemPtr { item, list_def, root }
    }
    pub fn item(&self) -> *mut MutItem { self.item }
    pub fn list_def(&self) -> *const ListDefObj{ self.list_def }
}

pub fn get_mil<T : From<MItemPtr>>(ps : MItemPtr, name : &str) -> Option<Option<MListPtr<T>>> {
    let (item, list_def) = unsafe { (&mut *ps.item, &*ps.list_def) };
    if let Some(ListDefValue::MilDef(md)) = list_def.default().get(name) {
        if let Some(ListSabValue::Mil(data)) = item.values_mut().get_mut(name) {
            if let Some(inner) = data {
                return Some(Some(MListPtr::new(inner.list_mut(), md.list_def(), ps.root)))
            } else {
                return Some(None)
            }
        }
    }
    return None
}

pub fn get_bool(ps : MItemPtr, name : &str) -> Option<Qv<bool>>{
    let (item,list_def) = unsafe{ (ps.item.as_ref().unwrap(), ps.list_def.as_ref().unwrap()) };
    if let Some(RustParam::Bool(b)) = get_param(item, list_def, name){
        Some(b.clone())
    } else{ None }
}

pub fn get_int(ps : MItemPtr, name : &str) -> Option<Qv<i64>>{
    let (item,list_def) = unsafe{ (ps.item.as_ref().unwrap(), ps.list_def.as_ref().unwrap()) };
    if let Some(RustParam::Int(b)) = get_param(item, list_def, name){
        Some(b.clone())
    } else{ None }
}

pub fn get_float(ps : MItemPtr, name : &str) -> Option<Qv<f64>>{
    let (item,list_def) = unsafe{ (ps.item.as_ref().unwrap(), ps.list_def.as_ref().unwrap()) };
    if let Some(RustParam::Float(b)) = get_param(item, list_def, name){
        Some(b.clone())
    } else{ None }
}

pub fn get_str(ps : MItemPtr, name : &str) -> Option<Qv<String>>{
    let (item,list_def) = unsafe{ (ps.item.as_ref().unwrap(), ps.list_def.as_ref().unwrap()) };
    if let Some(RustParam::String(b)) = get_param(item, list_def, name){
        Some(b.map(|s| s.str().to_string()))
    } else{ None }
}

pub fn get_int_array(ps : MItemPtr, name : &str) -> Option<Qv<Vec<i64>>>{
    let (item,list_def) = unsafe{ (&*ps.item, &*ps.list_def) };
    if let Some(RustParam::IntArray(b)) = get_param(item, list_def, name){
        Some(b.map(|s| s.vec().clone()))
    } else{
        None
    }
}

pub fn get_float_array(ps : MItemPtr, name : &str) -> Option<Qv<Vec<f64>>>{
    let (item,list_def) = unsafe{ (&*ps.item, &*ps.list_def) };
    if let Some(RustParam::FloatArray(b)) = get_param(item, list_def, name){
        Some(b.map(|s| s.vec().clone()))
    } else{
        None
    }
}

pub fn set_bool(ps : MItemPtr, name : &str, val : Qv<bool>) -> bool{
    let (item,def) = unsafe{ (ps.item.as_mut().unwrap(),ps.list_def.as_ref().unwrap()) };
    match item.set_sabun(def, name.to_string(), RustParam::Bool(val)) {
        Ok(_) => true,
        Err(_) => false,
    }
}
pub fn set_int(ps : MItemPtr, name : &str, val : Qv<i64>) -> bool{
    let (item, def) = unsafe{ (ps.item.as_mut().unwrap(), ps.list_def.as_ref().unwrap()) };
    match item.set_sabun(def,name.to_string(), RustParam::Int(val)){
        Ok(_) =>{ true },
        Err(e) => match e{
            SetSabunError::ParamNotFound =>{ false },
            SetSabunError::ParamTypeMismatch =>{ false },
            SetSabunError::QvTypeMismatch =>{ false },
        },
    }
}
pub fn set_float(ps : MItemPtr, name : &str, val : Qv<f64>) -> bool{
    let (item, def) = unsafe{ (ps.item.as_mut().unwrap(), ps.list_def.as_ref().unwrap()) };
    match item.set_sabun(def,name.to_string(), RustParam::Float(val)){
        Ok(_) =>{ true },
        Err(e) => match e{
            SetSabunError::ParamNotFound =>{ false },
            SetSabunError::ParamTypeMismatch =>{ false },
            SetSabunError::QvTypeMismatch =>{ false },
        },
    }
}
pub fn set_str(ps : MItemPtr, name : &str, val : Qv<String>) -> bool{
    let (item, def) = unsafe{ (ps.item.as_mut().unwrap(), ps.list_def.as_ref().unwrap()) };
    match item.set_sabun(def,name.to_string(), RustParam::String(val.into_map(|s| RustString::new(s)))){
        Ok(_) =>{ true },
        Err(e) => match e{
            SetSabunError::ParamNotFound =>{ false },
            SetSabunError::ParamTypeMismatch =>{ false },
            SetSabunError::QvTypeMismatch =>{ false },
        },
    }
}
pub fn set_int_array(ps : MItemPtr, name : &str, val : Qv<Vec<i64>>) -> bool{
    let (item, def) = unsafe{ (ps.item.as_mut().unwrap(), ps.list_def.as_ref().unwrap()) };
    match item.set_sabun(def,name.to_string(), RustParam::IntArray(val.into_map(|s| RustIntArray::new(s)))){
        Ok(_) =>{ true },
        Err(e) => match e{
            SetSabunError::ParamNotFound =>{ false },
            SetSabunError::ParamTypeMismatch =>{ false },
            SetSabunError::QvTypeMismatch =>{ false },
        },
    }
}
pub fn set_float_array(ps : MItemPtr, name : &str, val : Qv<Vec<f64>>) -> bool{
    let (item, def) = unsafe{ (ps.item.as_mut().unwrap(), ps.list_def.as_ref().unwrap()) };
    match item.set_sabun(def,name.to_string(), RustParam::FloatArray(val.into_map(|s| RustFloatArray::new(s)))){
        Ok(_) =>{ true },
        Err(e) => match e{
            SetSabunError::ParamNotFound =>{ false },
            SetSabunError::ParamTypeMismatch =>{ false },
            SetSabunError::QvTypeMismatch =>{ false },
        },
    }
}


pub fn get_param<'a>(item : &'a MutItem, def : &'a ListDefObj, name : &str) -> Option<&'a RustParam>{
    if let Some(ListSabValue::Param(p)) = item.values().get(name){
        Some(p)
    } else if let Some(ListDefValue::Param(p, _)) = def.default().get(name){
        Some(p)
    } else{
        None
    }
}



pub fn set_ref(ps : MItemPtr, list_name : &str, id : Qv<String>) -> bool{
    let (item, _def)= unsafe{ (ps.item.as_mut().unwrap(), ps.list_def.as_ref().unwrap()) };

    //if def.refs().refs().contains_key(list_name) == false{ return false; }

    item.refs_mut().insert(list_name.to_string(), RefSabValue::new(id));
    return true;
}


pub fn get_ref(ps : MItemPtr, list_name : &str) -> Option<Qv<CItemPtr>>{
    let qv = get_ref_id(ps, list_name)?;
    qv.opt_map(|id|{
        let data = super::root::get_table(RootObjectPtr::new(ps.root), list_name).unwrap();
        super::table::get_value(data, id)
    })
}

pub fn get_ref_id(ps : MItemPtr, list_name : &str) -> Option<Qv<String>>{
    let (item, list_def) = unsafe{ (ps.item.as_ref().unwrap(), ps.list_def.as_ref().unwrap()) };
    get_ref_id_imol(item.refs(), list_def, list_name)
}

pub fn get_enum(ps : MItemPtr) -> Option<(String, String)>{
    let item = unsafe{ ps.item.as_ref().unwrap() };
    get_enum_impl(item.refs())
}

pub fn set_enum(ps : MItemPtr, list_name : &str, id : &str) -> bool{
    let item = unsafe{ &mut *ps.item };
    item.refs_mut().clear();
    set_ref(ps, list_name, Qv::Val(id.to_string()))
}