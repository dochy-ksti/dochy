use crate::core::structs::{Qv, MetaTable, RefSabValue};
use crate::core::HashM;
use crate::diff::diff_error::DiffError;

pub(crate) fn apply_refs(refs : Vec<(usize, Qv<String>)>, meta : &MetaTable,
                  r : &mut HashM<String, RefSabValue>) -> Result<(), DiffError>{

    for (id, qv) in refs{
        let (key, _) = if let Some(v) = meta.get(id){ v } else{
            Err("meta is invalid apply_refs")?
        };
        r.insert(key.to_string(), RefSabValue::new(qv));
    }
    Ok(())
}