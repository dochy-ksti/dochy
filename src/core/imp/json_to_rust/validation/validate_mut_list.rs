use crate::core::error::CoreResult;
use crate::core::imp::json_to_rust::names::Names;
use crate::core::imp::json_to_rust::validation::validate_list_item::validate_list_item;
use crate::core::imp::structs::root_obj::RootObject;
use crate::core::imp::structs::list_def_obj::ListDefObj;
use crate::core::imp::structs::rust_list::MutItem;
use crate::core::imp::structs::linked_m::LinkedMap;

pub(crate) fn validate_mut_list(def : &ListDefObj, map : &LinkedMap<MutItem>, root : &RootObject,
                         can_use_old: bool, names : &Names) -> CoreResult<()>{
    for (idx, val) in map{
        let idx = format!("#{}", idx);
        let names = &names.append(&idx);
        validate_list_item(def, val.values(), val.refs(), root, can_use_old, names)?;
    }
    return Ok(());
}