use crate::imp::structs::rust_param::RustParam;
use crate::imp::structs::rust_list::{ConstListVal, MutListVal};
use crate::imp::structs::rust_value::{RustMemberType, RustValue};
use crate::imp::structs::var_type::VarType;
use crate::imp::structs::qv::QvType;
use crate::IdentityEqual;

#[derive(Debug, Clone)]
pub enum ListSabValue{
    Param(RustParam),
    //InnerData(InnerData),
    Cil(ConstListVal),
    ///MutInnerListだけundefinedになりうる
    Mil(Option<MutListVal>),
}

impl ListSabValue{
    pub(crate) fn type_num(&self) -> RustMemberType {
        use RustMemberType::*;

        match self{
            ListSabValue::Param(param) => param.type_num(),
            //ListSabValue::InnerData(_) => InnerData,
            ListSabValue::Cil(_) => Cil,
            ListSabValue::Mil(_) => Mil,
        }
    }

    ///ValueType::NormalとしてRustValue化する。これをjsonにすると、param_name : ["Num",null]とか言った感じになって、
    /// nullなのに?がない形だが、ListSabでは名前に?をつけるのは必須ではなく、むしろノイズになるので?は消す方が良いのでこうする
    /// nullの場合は"param_name?":["Num",null]のように?を補う実装があってもいいとは思うが、使いみちが今の所ない
    pub(crate) fn into_rust_value_for_json(self) -> RustValue{
        match self{
            //value側は名前に?とか!とかつけなくてよいのでValueType::Normal
            ListSabValue::Param(p) => RustValue::Param(p, VarType::Normal),
            //ListSabValue::InnerData(d) => RustValue::InnerData(d),
            ListSabValue::Cil(l) => RustValue::Cil(l),
            ListSabValue::Mil(m) => RustValue::Mil(m),
        }
    }

    pub(crate) fn qv_type(&self) -> QvType{
        match self{
            ListSabValue::Param(p) => p.qv_type(),
            ListSabValue::Mil(m) => if m.is_some(){ QvType::Val } else{ QvType::Undefined },
            _ => QvType::Val,
        }
    }
}

impl IdentityEqual for ListSabValue{
    fn identity_eq(&self, other: &Self) -> bool {
        match self{
            ListSabValue::Param(p) => if let ListSabValue::Param(p2) = other{ p.identity_eq(p2) } else{ false }
            ListSabValue::Mil(m) => if let ListSabValue::Mil(m2) = other{
                if let Some(m) = m{
                    if let Some(m2) = m2{
                        m.identity_eq(m2)
                    } else{ false }
                }
                else if m.is_none() && m2.is_none(){ true }
                else{ false }
            } else{ false },
            _ => true,
        }
    }
}