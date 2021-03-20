use crate::imp::structs::rust_param::RustParam;
use crate::imp::structs::rust_list::{ConstInnerList, MutInnerList};
use crate::imp::structs::rust_value::{RustValue, RustMemberType};
use crate::imp::structs::var_type::VarType;
use crate::imp::structs::qv::QvType;
use crate::imp::structs::list_def_obj::ListDefObj;
use crate::imp::structs::mil_def_obj::MilDefObj;

#[derive(Debug, PartialEq, Clone)]
pub enum ListDefValue{
    Param(RustParam, VarType),
    //InnerDataDef(ListDefObj),
    CilDef(ListDefObj),
    MilDef(MilDefObj),
}

#[derive(Debug, PartialEq, Clone)]
pub enum ListSabValue{
    Param(RustParam),
    //InnerData(InnerData),
    Cil(ConstInnerList),
    ///MutInnerListだけundefinedになりうる
    Mil(Option<MutInnerList>),
}

impl ListDefValue{
    pub(crate) fn into_rust_value(self) -> RustValue{
        match self{
            ListDefValue::Param(p,v) => RustValue::Param(p,v),
            //ListDefValue::InnerDataDef(d) => RustValue::InnerDataDef(d),
            ListDefValue::CilDef(l) => RustValue::CilDef(l),
            ListDefValue::MilDef(m) => RustValue::MilDef(m),
        }
    }

    pub(crate) fn acceptable(&self, other : &ListSabValue) -> bool{
        if self.type_num() == other.type_num(){
            if self.value_type().acceptable(&other.qv_type()){
                return true;
            }
        }
        false
    }

    // pub(crate) fn compatible(&self, other : &ListDefValue) -> bool{
    //     if self.type_num() == other.type_num(){
    //         if self.value_type().compatible(&other.value_type()){
    //             return true;
    //         }
    //     }
    //     false
    // }


    pub(crate) fn value_type(&self) -> VarType {
        match self{
            ListDefValue::Param(_param, vt) => vt.clone(),
            ListDefValue::MilDef(obj) => if obj.undefinable() { VarType::Undefiable } else{ VarType::Normal }
            _ => VarType::Normal,
        }
    }

    pub(crate) fn type_num(&self) -> RustMemberType {
        use RustMemberType::*;
        match self{
            ListDefValue::Param(param, _) => param.type_num(),
            //ListDefValue::InnerDataDef(_) => InnerData,
            ListDefValue::CilDef(_) => Cil,
            ListDefValue::MilDef(_) => Mil,
        }
    }

    // pub(crate) fn inner_def(&self) -> Option<&ListDefObj>{
    //     match self{
    //         //ListDefValue::InnerDataDef(d) => Some(d),
    //         ListDefValue::CilDef(d) => Some(d),
    //         ListDefValue::MilDef(obj) => Some(obj.list_def()),
    //         _ => None,
    //     }
    // }
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