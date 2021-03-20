use crate::imp::json_to_rust::json_name::{json_name, NameType};
use crate::HashM;
use crate::error::CoreResult;
use crate::imp::json_to_rust::validation::validate_root::validate_root;
use crate::imp::structs::var_type::VarType;
use crate::imp::structs::root_obj::RootObject;
use crate::imp::structs::root_value::RootValue;

///root.jsonからとったRootに、各ファイルからとった個別のメンバを混ぜる。ファイルはアルファベット順に、root.jsonの末尾に加わっていく
pub(crate) fn construct_root(root : RootObject, map : HashM<String, RootValue>, validation : bool) -> CoreResult<RootObject>{
    let (default, sabun, old, meta) = root.deconstruct();
    let mut default = default;
    let mut vec : Vec<(String, RootValue)>= map.into_iter().collect();
    vec.sort_by(|(a,_),(b,_)| a.cmp(b));
    for (key, value) in vec{
        let name = json_name(&key).ok_or_else(|| format!("filename:{} is not a valid name", &key))?;
        match name {
            NameType::Name(name, VarType::Normal) => {
                let id = default.len();
                default.insert(name, (id,value));
            },
            _=>{ Err(format!("{} is not a valid name", &key))?; }
        }
    }
    let root = RootObject::construct(default, sabun, old, meta);
    if validation{
        validate_root(&root, false)?
    }

    return Ok(root);
}