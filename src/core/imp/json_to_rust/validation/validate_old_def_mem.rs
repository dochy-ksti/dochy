use crate::core::{HashM};
use crate::core::imp::json_to_rust::names::Names;
use crate::core::error::CoreResult;
use crate::core::imp::structs::rust_list::ConstItem;
use crate::core::imp::structs::root_value::RootValue;
use crate::core::imp::structs::list_def_obj::ListDefMap;
use crate::core::imp::structs::ref_def_obj::RefDefMap;
use crate::core::imp::structs::util::hash_m::HashS;

pub(crate) fn validate_old_root_def_mem(old : &HashS<String>, map : &HashM<String, (usize, RootValue)>, names : &Names) -> CoreResult<()>{
    for name in old{
        if map.contains_key(name) == false{
            Err(format!("{} Old {} does not exist in the Default", names, name))?
        }
    }
    Ok(())
}

pub(crate) fn validate_old_list_def_mem(old : &HashS<String>, map : &ListDefMap, names : &Names) -> CoreResult<()>{
    for name in old{
        if map.contains_key(name) == false{
            Err(format!("{} Old {} does not exist in the Default", names, name))?
        }
    }
    Ok(())
}

pub(crate) fn validate_old_ref_def(old : &HashS<String>, ref_def : &RefDefMap, names : &Names) -> CoreResult<()>{
    for name in old{
        if ref_def.contains_key(name) == false{
            Err(format!("{} Old {} does not exist in the Ref", names, name))?
        }
    }
    Ok(())
}

pub(crate) fn validate_old_table_id(old : &HashS<String>, data_map : &HashM<String, ConstItem>, names : &Names) -> CoreResult<()> {
    for name in old{
        if data_map.contains_key(name) == false{
            Err(format!("{} Old's ID {} does not exist in the Data", names, name))?
        }
    }
    Ok(())
}