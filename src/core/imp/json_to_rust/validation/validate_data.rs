use crate::core::{HashM};
use crate::core::error::CoreResult;
use crate::core::imp::json_to_rust::names::Names;
use crate::core::imp::json_to_rust::validation::validate_list_item::validate_list_item;
use crate::core::imp::json_to_rust::validation::validate_old_def_mem::validate_old_table_id;
use crate::core::imp::structs::rust_list::ConstItem;
use crate::core::imp::structs::root_obj::RootObject;
use crate::core::imp::structs::list_def_obj::ListDefObj;
use crate::core::imp::structs::util::hash_m::HashS;

pub(crate) fn validate_table(def : &ListDefObj, data_map : &HashM<String, ConstItem>, root : &RootObject, old : &HashS<String>,
                      can_use_old: bool, names : &Names) -> CoreResult<()>{
    validate_old_table_id(old, data_map, names)?;

    for (name, val) in data_map{
        //name==old なものがあっても別にかまわない。消すと互換性が崩れるだろう
        validate_list_item(def, val.values(), val.refs(), root, can_use_old, &names.append(name))?
    }
    return Ok(());
}