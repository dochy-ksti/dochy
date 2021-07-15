use crate::imp::structs::var_type::VarType;
use crate::imp::structs::rust_list::{ConstTable, ConstList, MutList, ConstListVal, MutListVal};
use crate::imp::structs::root_value::RootValue;
use crate::imp::structs::list_value::{ListDefValue, ListSabValue};
use crate::imp::structs::rust_param::RustParam;
use crate::imp::structs::list_def_obj::ListDefObj;
use crate::imp::structs::mut_list_def::MutListDef;


#[derive(Debug, Clone)]
pub enum RustValue{
    Param(RustParam, VarType),
    Table(ConstTable),
    CList((ListDefObj, ConstListVal)),
    MList((MutListDef, Option<MutListVal>)),
    Cil(ConstListVal),
    ///MutInnerListだけundefinedになりうる
    Mil(Option<MutListVal>),
    //InnerDataDef(ListDefObj),
    CilDef(ListDefObj),
    MilDef(MutListDef),
}

#[repr(u64)]
#[derive(Debug, PartialEq, Clone)]
pub enum RustMemberType {
    Bool, Float, Int, Str, IntArray, FloatArray, Binary,
    Table, CList, MList, Cil, Mil
}

impl RustValue{
    pub(crate) fn into_root_value(self) -> Result<RootValue, String>{
        let v = match self{
            RustValue::Param(p,v) => RootValue::Param(p,v),
            RustValue::Table(d) => RootValue::Table(d),
            RustValue::CList(l) => RootValue::CList(l),
            RustValue::MList(m) => RootValue::MList(m),
            _ =>{ return Err(self.type_string()); },
        };
        Ok(v)
    }

    pub(crate) fn into_root_value2(self, name : &str) -> crate::error::CoreResult<RootValue>{
        match self.into_root_value(){
            Ok(a) => Ok(a),
            Err(s) =>{ Err(format!("{} the root obj can't have {}", name, s))? }
        }
    }

    pub(crate) fn into_list_def_value(self) -> Result<ListDefValue, String>{
        let v = match self{
            RustValue::Param(p,v) => ListDefValue::Param(p,v),
            //RustValue::InnerDataDef(d) => ListDefValue::InnerDataDef(d),
            RustValue::CilDef(l) => ListDefValue::CilDef(l),
            RustValue::MilDef(m) => ListDefValue::MilDef(m),
            _ =>{ return Err(self.type_string()); },
        };
        Ok(v)
    }

    ///失敗時はtype_stringを返す
    pub(crate) fn into_list_sab_value(self) -> Result<ListSabValue, String>{
        let v = match self{
            RustValue::Param(p,_v) => ListSabValue::Param(p),
            //RustValue::InnerData(d) => ListSabValue::InnerData(d),
            RustValue::Cil(l) => ListSabValue::Cil(l),
            RustValue::Mil(m) => ListSabValue::Mil(m),
            _ =>{ return Err(self.type_string()); },
        };
        Ok(v)
    }

    pub(crate) fn type_string(&self) -> String{
        let s = match self{
            RustValue::Param(_, _) => "Param",
            RustValue::Table(_) => "Table",
            RustValue::CList(_) => "CList",
            RustValue::MList(_) => "MList",
            RustValue::Cil(_) => "Cil",
            RustValue::Mil(_) => "Mil",
            RustValue::CilDef(_) => "CilDef",
            RustValue::MilDef(_) => "MilDef",
        };
        s.to_string()
    }
}

