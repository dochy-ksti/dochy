use crate::json5::{JVal, Span};
use crate::core::imp::json_to_rust::names::Names;
use crate::core::error::CoreResult;
use crate::core::imp::json_to_rust::json_obj_to_rust::json_obj_to_rust;
use crate::core::imp::json_to_rust::json_name::json_simple_name;
use crate::core::imp::json_to_rust::tmp::tmp_obj::TmpRefs;
use crate::core::{HashM, HashMt};
use crate::core::imp::structs::ref_value::RefValue;
use crate::core::imp::structs::rust_value::{RustValue};
use crate::core::imp::structs::qv::Qv;
use crate::core::imp::structs::rust_param::RustParam;
use linked_hash_map::LinkedHashMap;
//use linked_hash_map::LinkedHashMap;

pub(crate) fn get_ref(v : &LinkedHashMap<String, JVal>, span : &Span, names : &Names) -> CoreResult<TmpRefs> {
    let obj = json_obj_to_rust(v, true, span, names)?;
    if obj.refs.map.len() != 0 {
        Err(format!(r#"{} Ref can't be declared in a Ref object {}"#, span.line_str(), names))?
    }
    if obj.id.is_some() {
        Err(format!(r#"{} ID can't be declared in a Ref object {}"#, span.line_str(), names))?
    }
    if obj.include.len() != 0{
        Err(format!(r#"{} Include can't be declared in a Ref object {}"#, span.line_str(), names))?
    }


    let mut map: HashM<String, (usize, RefValue)> = HashMt::with_capacity(obj.default.len());
    for (idx, (k, (_, v))) in obj.default.iter().enumerate() {
        match v {
            RustValue::Param(RustParam::String(v), vt) => {
                match v {
                    Qv::Val(s) =>{
                        if json_simple_name(s.str()).is_none() && s.str().is_empty() == false{
                            //undefinedは勝手にいれちゃいけないから、エラーメッセージには表示しないが、別に入れられる
                            Err(format!(r#"{} {} Ref's value must be a simple name, null or empty {}"#, span.line_str(), s, names))?
                        }
                    },
                    _ =>{},
                }
                map.insert(k.to_string(), (idx, RefValue::new(v.map(|s| s.str().to_string()), vt.clone())));
            },
            _ => {
                Err(format!(r#"{} {} Ref's value must be a string or null {}"#, span.line_str(), k, names))?
            }
        }
    }
    return Ok(TmpRefs{ map, old : obj.old, is_enum : false, span : span.clone() });
}


//fn get_name(name : &str) -> Option<String>{
//    let name_type = json_name(name)?;
//    match &name_type{
//        NameType::Normal => return Some(name_type),
//        _ => return None,
//    };
//}