use crate::core::structs::{RootObject, RootValue, MetaValue};
use crate::diff::imp::structs_read::RootDiffR;
use crate::diff::imp::apply::apply_list_diff::apply_list_diff;
use crate::diff::diff_error::DiffError;

pub(crate) fn apply_root_diff(root : &mut RootObject, diff : RootDiffR) -> Result<(), DiffError>{
    let (params, lists) = diff.deconstruct();
    let (default, sabun, _old, meta)
        = root.mut_refs();
    //let (mut default, mut sabun, old, meta) = root.deconstruct();
    for (id, p) in params{
        let (key, _meta_val) = if let Some(v) = meta.get(id){ v } else{
            unreachable!("meta is not valid apply_root_diff")
        };
        sabun.insert(key.to_string(), p);
    }
    for (id, list_diff) in lists{
        let (key, meta_val) = if let Some(v) = meta.get(id){ v } else{
            unreachable!("list meta is not valid apply_root_diff")
        };
        let (_id, v) = if let Some(v) = default.get_mut(key){ v } else{
            unreachable!("invalid default apply_root_diff")
        };
        let tables = if let MetaValue::MList(tables) = meta_val{ tables } else{
            unreachable!("invalid meta apply_root_diff")
        };
        match v{
            RootValue::MList(m) =>{ apply_list_diff(m.list_mut(), list_diff.unwrap(), tables)? },
            _ =>{ unreachable!("invalid RootType apply_root_diff") },
        }
    }
    Ok(())
}

// pub(crate ) fn apply_root_diff(root : RootObject, diff : RootDiffR) -> Result<RootObject, DiffError>{
//     let (params, lists) = diff.deconstruct();
//     let (mut default, mut sabun, old, meta) = root.deconstruct();
//     for (id, p) in params{
//         let (key, _meta_val) = if let Some(v) = meta.get(id){ v } else{
//             unreachable!("meta is not valid apply_root_diff")
//         };
//         sabun.insert(key.to_string(), p);
//     }
//     for (id, list_diff) in lists{
//         let (key, meta_val) = if let Some(v) = meta.get(id){ v } else{
//             unreachable!("list meta is not valid apply_root_diff")
//         };
//         let (_id, v) = if let Some(v) = default.get_mut(key){ v } else{
//             unreachable!("invalid default apply_root_diff")
//         };
//         let tables = if let MetaValue::MList(tables) = meta_val{ tables } else{
//             unreachable!("invalid meta apply_root_diff")
//         };
//         match v{
//             RootValue::MList(m) =>{ apply_list_diff(m.list_mut(), list_diff.unwrap(), tables)? },
//             _ =>{ unreachable!("invalid RootType apply_root_diff") },
//         }
//     }
//     Ok(RootObject::construct(default, sabun, old, meta))
// }