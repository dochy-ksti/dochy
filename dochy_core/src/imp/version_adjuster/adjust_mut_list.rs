use crate::error::CoreResult;
use crate::imp::version_adjuster::adjust_mut_list_item_sabun::adjust_mut_list_item_sabun;
use crate::imp::json_to_rust::names::Names;
use crate::imp::version_adjuster::adjust_mut_list_item_ref::adjust_mut_list_item_ref;
use crate::imp::structs::rust_list::{MutItem, MutList, MutListVal};
use crate::imp::structs::list_def_obj::ListDefObj;
use crate::imp::structs::linked_m::LinkedMap;


pub(crate) fn adjust_mut(def : &ListDefObj, old_list : LinkedMap<MutItem>, names : &Names) -> CoreResult<LinkedMap<MutItem>>{
    //let mut counter : u64 = 0;
    //let mut result : HashM<u64, Box<MutListItem>> = HashMt::with_capacity(old_list.len());
    let mut result : Vec<(u64, MutItem)> = Vec::with_capacity(old_list.len());
    let next_id = old_list.next_id();
    for (id, value) in old_list{
        let (sabun, refs) = value.deconstruct();
        let new_sabun = adjust_mut_list_item_sabun(def, *sabun, names)?;
        let new_refs = adjust_mut_list_item_ref(def.refs(), *refs, names)?;
        result.push((id, MutItem::new(new_sabun, new_refs)));
    }
    return Ok(LinkedMap::construct(result, next_id));
}

pub(crate) fn adjust_mut_list(new : MutList, old : MutList, names : &Names) -> CoreResult<MutList>{
    let (_,old_list) = old.deconstruct();

    let new_list = adjust_mut(new.default(), old_list, names)?;
    //let next_id = new_list.len() as u64;
    let(default,_) = new.deconstruct();
    Ok(MutList::new(default, new_list))
}

pub(crate) fn adjust_mut_inner_list(def : &ListDefObj, old : MutListVal, names : &Names) -> CoreResult<MutListVal>{
    let old_list = old.deconstruct();

    let new_list = adjust_mut(def, old_list, names)?;
    //let next_id = new_list.len() as u64;
    Ok(MutListVal::new(new_list))
}