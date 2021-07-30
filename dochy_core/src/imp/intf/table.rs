use crate::imp::structs::rust_list::{ConstTable, ConstItem};
use crate::{HashM};
use crate::imp::intf::citem::CItemPtr;
use crate::imp::structs::list_def_obj::ListDefObj;
use crate::imp::structs::util::hash_m::HashS;
use crate::imp::structs::root_def_obj::RootDefObj;

// pub fn get_member_desc(root : *const ConstData) -> MemberDescs{
//     let root = unsafe{ root.as_ref().unwrap() };
//     get_list_def_desc(root.default())
// }

// pub fn get_ref_desc(root : *const ConstData) -> RefDescs{
//     let root = unsafe{ root.as_ref().unwrap() };
//     get_ref_def_desc(root.default().refs())
// }

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct TablePtr<'a> {
    ptr : &'a ConstTable,
    root : &'a RootDefObj,
}
impl<'a> TablePtr<'a> {
    pub fn new<'b>(ptr : &'b ConstTable, root : &'b RootDefObj) -> TablePtr<'b> { TablePtr { ptr, root } }
}

#[derive(Debug, PartialEq, Clone)]
pub struct DataKV<'a> {
    is_old : bool,
    id : String,
    item : &'a ConstItem,
}

impl<'a> DataKV<'a> {
    pub(crate) fn new(is_old : bool, id : String, item : &ConstItem) -> DataKV { DataKV { is_old, id, item }}
    pub fn is_old(&self) -> bool { self.is_old }
    pub fn id(&self) -> &'a str{ self.id.as_str() }
    pub fn item(&self) -> &'a ConstItem { self.item }
}

#[derive(Debug, PartialEq, Clone)]
pub struct DataKVs<'a> {
    items : Vec<DataKV<'a>>,
    list_def : &'a ListDefObj,
}

impl<'a> DataKVs<'a> {
    pub(crate) fn new<'b>(items : Vec<DataKV<'b>>, list_def : &'b ListDefObj) -> DataKVs<'b> { DataKVs { items, list_def }}
    pub fn items(&self) -> &[DataKV]{ &self.items }
    pub fn def(&self) -> &'a ListDefObj{ self.list_def }
}

pub fn get_kvs(data : TablePtr) -> DataKVs {
    let data = unsafe{ data.ptr.as_ref().unwrap() };
    get_kvs_impl(data.default(), data.list(), data.old())
}

pub fn get_kvs_impl<'a>(list_def : &'a ListDefObj, data : &'a HashM<String, ConstItem>, old : &'a HashS<String>) -> DataKVs<'a> {
    DataKVs::new(data.iter().map(|(k,v)|
        DataKV::new(old.contains(k), k.to_string(), v)).collect(),
                 list_def)
}

pub fn get_value<'a>(data : TablePtr<'a>, id : &str) -> Option<CItemPtr<'a>>{
    let d = unsafe{data.ptr.as_ref().unwrap()};
    get_value_impl(d.list(), d.default(), id, data.root)
}

pub fn get_value_impl<'a>(data : &'a HashM<String, ConstItem>, list_def : &'a ListDefObj, id : &str, root_def : &'a RootDefObj) -> Option<CItemPtr<'a>>{
    data.get(id).map(|i| CItemPtr::new(i, list_def, root_def))
}

