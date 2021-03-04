use crate::core::imp::structs::qv::{Qv, QvType};
use crate::core::imp::structs::rust_string::{RustString };
use crate::core::imp::structs::rust_value::RustMemberType;
use crate::core::imp::structs::rust_array::{RustArray, RustIntArray, RustFloatArray, };
use crate::core::imp::structs::array_type::ArrayType;

#[derive(Debug, PartialEq, Clone)]
pub enum RustParam{
    Bool(Qv<bool>),
    Float(Qv<f64>),
    Int(Qv<i64>),
    String(Qv<RustString>),
    IntArray(Qv<RustIntArray>),
    FloatArray(Qv<RustFloatArray>),
    //StrArray(Qv<RustStrArray>),
    //Num2Array(Qv<RustNum2Array>)
}


impl RustParam {
    pub fn qv_type(&self) -> QvType {
        match self {
            RustParam::Bool(b) => b.qv_type(),
            RustParam::Float(f) => f.qv_type(),
            RustParam::Int(i) => i.qv_type(),
            RustParam::String(s) => s.qv_type(),
            RustParam::IntArray(a) => a.qv_type(),
            RustParam::FloatArray(a) => a.qv_type(),
            //RustParam::StrArray(a) => a.qv_type(),
            //RustParam::Num2Array(a) => a.qv_type(),
        }
    }

    pub(crate) fn type_num(&self) -> RustMemberType {
        use RustMemberType::*;
        match self {
            RustParam::Bool(_) => Bool,
            RustParam::Float(_) => Float,
            RustParam::Int(_) => Int,
            RustParam::String(_) => Str,
            RustParam::FloatArray(_) => FloatArray,
            RustParam::IntArray(_) => IntArray,
            //RustParam::StrArray(_) => StrArray,
            //RustParam::Num2Array(_) => Num2Array,
        }
    }


    pub(crate) fn acceptable(&self, other: &Self) -> bool {
        if self.type_num() != other.type_num() {
            return false;
        }
        return true;
    }

    ///型情報を維持したままundefinedに変える
    pub(crate) fn to_undefined(&self) -> Self {
        match self {
            RustParam::Bool(_) => RustParam::Bool(Qv::Undefined),
            RustParam::Float(_) => RustParam::Float(Qv::Undefined),
            RustParam::Int(_) => RustParam::Int(Qv::Undefined),
            RustParam::String(_) => RustParam::String(Qv::Undefined),
            RustParam::FloatArray(_) => RustParam::FloatArray(Qv::Undefined),
            RustParam::IntArray(_) => RustParam::IntArray(Qv::Undefined),
            //RustParam::StrArray(_) => RustParam::StrArray(Qv::Undefined),
            //RustParam::Num2Array(_) => RustParam::Num2Array(Qv::Undefined)
        }
    }

    pub(crate) fn to_rust_array(&self) -> Option<(RustArray, ArrayType)>{
        match self{
            RustParam::FloatArray(a) => Some((RustArray::from_float_array(a), ArrayType::Float)),
            RustParam::IntArray(a) => Some((RustArray::from_int_array(a), ArrayType::Int)),
            //RustParam::StrArray(a) => Some((RustArray::from_str_array(a), ArrayType::String)),
            //RustParam::Num2Array(a) => Some((RustArray::from_num2_array(a), ArrayType::Num2)),
            _ => None,
        }
    }

    pub(crate) fn to_float(&self) -> Option<f64>{
        if let RustParam::Float(Qv::Val(s)) = self { Some(*s) } else{ None }
    }

    pub(crate) fn to_int(&self) -> Option<i64>{
        if let RustParam::Int(Qv::Val(s)) = self { Some(*s) } else{ None }
    }
}