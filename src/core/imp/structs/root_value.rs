use crate::core::imp::structs::rust_param::RustParam;
use crate::core::imp::structs::var_type::VarType;
use crate::core::imp::structs::rust_list::{ConstTable, ConstList, MutList};
use crate::core::imp::structs::rust_value::RustValue;

#[derive(Debug, PartialEq, Clone)]
pub enum RootValue{
    Param(RustParam, VarType),
    Table(ConstTable),
    CList(ConstList),
    MList(MutList),
}

impl RootValue{
    // pub(crate) fn list_def(&self) ->  Option<&ListDefObj> {
    //     match self {
    //         RootValue::Table(d) => Some(d.default()),
    //         RootValue::CList(d) => Some(d.default()),
    //         RootValue::MList(d) => Some(d.default()),
    //         _ => None,
    //     }
    // }


    pub fn into_rust_value(self) -> RustValue{
        match self{
            RootValue::Param(p,v) => RustValue::Param(p,v),
            RootValue::Table(d) => RustValue::Table(d),
            RootValue::CList(l) => RustValue::CList(l),
            RootValue::MList(m) => RustValue::MList(m),
        }
    }
}
