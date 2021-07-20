use crate::imp::json_to_rust::json_name::{json_name, NameType};
//use crate::HashM;
use crate::error::CoreResult;
use crate::imp::json_to_rust::validation::validate_root::validate_root;
use crate::imp::structs::var_type::VarType;
use crate::imp::structs::root_obj::RootObject;
use crate::imp::structs::root_value::RootValue;
use crate::imp::structs::util::hash_m::HashM;
use crate::imp::structs::root_sab_value::RootSabValue;
use crate::imp::structs::root_def_obj::RootDefObj;
use std::sync::Arc;
use crate::imp::structs::meta_table::MetaTable;

///root.jsonからとったRootに、各ファイルからとった個別のメンバを混ぜる。ファイルはアルファベット順に、root.jsonの末尾に加わっていく
pub(crate) fn construct_root(root : RootObject, vec : Vec<(String, RootValue, Option<RootSabValue>)>, validation : bool) -> CoreResult<RootObject>{
    let (default_v, sabun_v, old, _meta) = root.deconstruct();
    let mut default_v = default_v;
    let default = Arc::make_mut(&mut default_v).def_mut();
    let mut sabun_v = sabun_v;
    let mut sabun = Arc::make_mut(&mut sabun_v);
    let mut vec = vec;
    vec.sort_by(|(a,_,_),(b,_,_)| a.cmp(b));
    for (name, value, sab) in vec {
        let id = default.len();
        default.insert(name.to_string(), (id, value));
        if let Some(sab) = sab{
            sabun.insert(name, sab);
        }
    }
    let meta = MetaTable::from_root(default_v.def());
    let root = RootObject::construct(default_v, sabun_v, old, Arc::new(meta));
    if validation{
        validate_root(&root, false)?
    }

    return Ok(root);
}