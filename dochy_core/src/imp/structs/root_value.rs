use crate::imp::structs::rust_param::RustParam;
use crate::imp::structs::var_type::VarType;
use crate::imp::structs::rust_list::{ConstTable};
use crate::imp::structs::rust_value::RustValue;
use crate::IdentityEqual;
use crate::imp::structs::mut_list_def::MutListDef;
use crate::imp::structs::list_def_obj::ListDefObj;
use crate::error::CoreResult;
use crate::imp::structs::list_sab_value::ListSabValue;

#[derive(Debug, Clone)]
pub enum RootValue{
    Param(RustParam, VarType),
    Table(ConstTable),
    CList(ListDefObj),
    MList(MutListDef),
}

impl RootValue{

    pub fn into_rust_value(self, sab : ListSabValue) -> CoreResult<RustValue>{
        match self{
            RootValue::Param(p,v) => Ok(RustValue::Param(p,v)),
            RootValue::Table(d) => Ok(RustValue::Table(d)),
            RootValue::CList(d) =>{
                if let ListSabValue::Cil(c) = sab{
                    Ok(RustValue::CList((d,c)))
                }  else{
                    Err("unmatched Const List")?
                }
            },
            RootValue::MList(d) =>{
                if let ListSabValue::Mil(m) = sab {
                    Ok(RustValue::MList((d, m)))
                } else{
                    Err("unmatched Mut List")?
                }
            }
        }
    }
}

impl IdentityEqual for RootValue{
    fn identity_eq(&self, _other: &Self) -> bool {
        //todo: rootvalue に　identity_eq 必要ないからちゃんとけして
        match self{
            //RootValue::Param(p, _) => if let RootValue::Param(p2, _) = other{ p.identity_eq(p2) } else{ false },
            //RootValue::MList(m) => if let RootValue::MList(m2) = other{ m.identity_eq(m2)} else{ false },
            _ => true, //constのものが違う可能性は考えない。それはバージョンが違うということでありidentity_eqはそれを考えない
        }
    }
}