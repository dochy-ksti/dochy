use crate::error::CoreResult;
use crate::imp::rust_to_json::list::value_map_to_json::value_map_to_json;
use crate::imp::rust_to_json::list::tmp_json_list::{btree_map, btree_set};
use crate::imp::rust_to_json::string_set_to_json::{string_set_to_json_short};
use crate::{HashM, HashMt};
use crate::imp::structs::rust_value::{RustValue};
use crate::imp::structs::root_obj::RootObject;
use crate::imp::structs::my_json::Value;
use crate::imp::structs::root_value::RootValue;

///本来デフォルト値と差分が保存されているのだが、見やすくするためにまとめてデフォルト値にしてしまう。
///デフォルト値も差分も全部Json化したいユースケースもあるかもしれない・・・？

pub fn root_to_json_new_default(obj : &RootObject) -> CoreResult<Value> {
    let mut result : HashM<String,RustValue> = HashMt::with_capacity(obj.default().len());
    let default = obj.default().clone();
    let mut sabun = obj.sabun().clone();
    let old = obj.old().clone();

    for (name, (_id, val)) in default{
        if let RootValue::Param(p,vt) = val{
            if let Some(sab_param) = sabun.remove(&name){
                result.insert(name, RustValue::Param(sab_param, vt));
            } else{
                result.insert(name, RustValue::Param(p, vt));
            }
        } else{
            result.insert(name, val.into_rust_value());
        }
    }

    let mut map = value_map_to_json(&btree_map(&result));
    map.insert( "Old".to_string(), string_set_to_json_short(&btree_set(&old)));

    return Ok(Value::Map(map));
}

