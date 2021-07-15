use crate::imp::structs::rust_param::RustParam;
use crate::imp::structs::var_type::VarType;
use crate::imp::structs::rust_list::{ConstTable, ConstList, MutList};
use crate::imp::structs::rust_value::RustValue;
use crate::IdentityEqual;
use crate::imp::structs::mut_list_def::MutListDef;
use crate::imp::structs::list_value::ListSabValue;
use crate::imp::structs::list_def_obj::ListDefObj;

#[derive(Debug, Clone)]
pub enum RootValue{
    Param(RustParam, VarType),
    Table(ConstTable),
    CList(ListDefObj),
    MList(MutListDef),
}

impl RootValue{

    pub fn into_rust_value(self, sabValue : ListSabValue) -> RustValue{
        match self{
            RootValue::Param(p,v) => RustValue::Param(p,v),
            RootValue::Table(d) => RustValue::Table(d),
            RootValue::CList(d) =>{
                if let ListSabValue::Cil(c) = sabValue{
                    RustValue::CList((d,c))
                }
            },

            RootValue::MList(d) =>{
                if let ListSabValue::Mil(m) = sabValue {
                    RustValue::MList((d, m))
                }
            }
        }
    }
}

impl IdentityEqual for RootValue{
    fn identity_eq(&self, other: &Self) -> bool {
        match self{
            RootValue::Param(p, _) => if let RootValue::Param(p2, _) = other{ p.identity_eq(p2) } else{ false },
            RootValue::MList(m) => if let RootValue::MList(m2) = other{ m.identity_eq(m2)} else{ false },
            _ => true, //constのものが違う可能性は考えない。それはバージョンが違うということでありidentity_eqはそれを考えない
        }
    }
}