use crate::core::imp::json_to_rust::tmp::tmp_obj::IdValue;
use crate::json5::{JVal};

pub(crate) fn get_id(v : &JVal) -> Option<IdValue> {
    match v {
        JVal::String(s, _) => Some(IdValue::Str(s.to_string())),
        JVal::Double(d, _) => Some(IdValue::Num(*d as u64)),
        _ => None
    }
}