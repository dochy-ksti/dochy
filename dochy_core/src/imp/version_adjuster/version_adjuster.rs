use crate::imp::json_to_rust::validation::validate_root::validate_root;
use crate::imp::version_adjuster::adjust_mut_list::adjust_mut_list;
use crate::{HashM, HashMt};
use crate::error::CoreResult;
use crate::imp::json_to_rust::names::Names;
use crate::imp::structs::root_obj::RootObject;
use crate::imp::structs::root_value::RootValue;
use crate::structs::MetaTable;

/// paramのsabunがあれば上書き、mut_listはoldのものを全部入れ、（あるなら）newの方のものは全削除して入れ替える
/// 基本的に、新バージョンのjsonと旧バージョンのデータが有り、旧バージョンのデータはRootのsabunとMutListには変更が加えられているだろう
/// Defaultが更新されるので、undefinedが設定される。
pub fn adjust_versions(new : RootObject, old : RootObject, validation : bool) -> CoreResult<RootObject>{

    let (def, sabun, old_hash, _meta) = new.deconstruct();
    let mut sabun = sabun;
    let mut new_map :HashM<String, (usize, RootValue)> = HashMt::with_capacity(def.len());

    let (old_def,old_sabun, _, _) = old.deconstruct();
    let mut old_sabun = old_sabun;
    let mut old_def = old_def;

    for (def_key, (id, def_value)) in *def{
        match def_value{
            RootValue::Param(p,v) =>{
                let undef = if v.undefiable(){
                    if old_def.contains_key(&def_key) == false{
                        sabun.insert(def_key.to_string(),p.to_undefined());
                        true
                    } else {
                        false
                    }
                } else{
                    false
                };

                if undef == false {
                    if let Some(param) = old_sabun.remove(&def_key) {
                        sabun.insert(def_key.to_string(), param);
                    }
                }
                new_map.insert(def_key,(id, RootValue::Param(p,v)));
            },
            RootValue::MList(m) =>{
                if let Some((_,RootValue::MList(old_m))) = old_def.remove(&def_key){
                    let new_m = adjust_mut_list(m, old_m, &Names::new(&def_key))?;
                    new_map.insert(def_key, (id, RootValue::MList(new_m)));
                } else{
                    new_map.insert(def_key, (id, RootValue::MList(m)));
                }
            }
            _ =>{
                //MutとParam以外にadjustする対象はないはず
                new_map.insert(def_key, (id, def_value));
            },
        }
    }
    let new_def = Box::new(new_map);
    let new_meta = MetaTable::from_root(new_def.as_ref());
    let new = RootObject::construct(new_def, sabun, old_hash, Box::new(new_meta));

    if validation{
        validate_root(&new, true)?
    }
    return Ok(new);
}

