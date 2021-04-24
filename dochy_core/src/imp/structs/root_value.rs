use crate::imp::structs::rust_param::RustParam;
use crate::imp::structs::var_type::VarType;
use crate::imp::structs::rust_list::{ConstTable, ConstList, MutList};
use crate::imp::structs::rust_value::RustValue;

#[derive(Debug, Clone)]
pub enum RootValue{
    Param(RustParam, VarType),
    Table(ConstTable),
    CList(ConstList),
    MList(MutList),
}

impl RootValue{

    pub fn into_rust_value(self) -> RustValue{
        match self{
            RootValue::Param(p,v) => RustValue::Param(p,v),
            RootValue::Table(d) => RustValue::Table(d),
            RootValue::CList(l) => RustValue::CList(l),
            RootValue::MList(m) => RustValue::MList(m),
        }
    }
}
