use crate::core::structs::{RustParam, MetaParam, VarType, QvType, RustIntArray, RustFloatArray};
use crate::compaction::kval_enum::KVal;
use crate::compaction::basic_compaction::{comp_int, comp_double, comp_str};
use crate::diff::diff_error::DiffError;

pub(crate) fn write_param(param : &RustParam, meta : &MetaParam, r : &mut Vec<KVal>) -> Result<(), DiffError>{
    match meta.var_type(){
        VarType::Normal =>{
            match param.qv_type() {
                QvType::Val => {
                    write_param2(param, r)
                },
                QvType::Null =>{
                    Err("invalid null")?
                },
                QvType::Undefined =>{
                    Err("invalid undefined")?
                }
            }
        },
        VarType::Nullable =>{
            match param.qv_type(){
                QvType::Val =>{
                    r.push(KVal::Bit(true));
                    write_param2(param, r)
                },
                QvType::Null =>{
                    r.push(KVal::Bit(false));
                    Ok(())
                },
                QvType::Undefined =>{ Err("invalid undefined")? }
            }
        },
        VarType::Undefiable =>{
            match param.qv_type(){
                QvType::Val =>{
                    r.push(KVal::Bit(true));
                    write_param2(param, r)
                },
                QvType::Undefined =>{
                    r.push(KVal::Bit(false));
                    Ok(())
                },
                QvType::Null =>{ Err("invalid null")? }
            }
        },
        VarType::UndefNullable =>{
            match param.qv_type(){
                QvType::Val =>{
                    r.push(KVal::Bit(true));
                    write_param2(param, r)
                },
                QvType::Null =>{
                    r.push(KVal::Bit(false));
                    r.push(KVal::Bit(true));
                    Ok(())
                },
                QvType::Undefined =>{
                    r.push(KVal::Bit(false));
                    r.push(KVal::Bit(false));
                    Ok(())
                },
            }
        }
    }
}

fn write_param2(param : &RustParam, r : &mut Vec<KVal>) -> Result<(), DiffError>{
    match param{
        RustParam::Bool(i) => r.push(KVal::Bit(*i.value()?)),
        RustParam::Int(i) => r.push(comp_int(*i.value()?)),
        RustParam::Float(f) => r.push(comp_double(*f.value()?)),
        RustParam::String(s) => r.push( comp_str(s.value()?.to_string())),
        RustParam::IntArray(s) => write_int_array(s.value()?, r),
        RustParam::FloatArray(s) => write_float_array(s.value()?, r),
    }
    Ok(())
}

fn write_int_array(a : &RustIntArray, r : &mut Vec<KVal>){
    let vec = a.vec();
    r.push(comp_int(vec.len() as i64));
    for i in vec{
        r.push(comp_int(*i));
    }
}

fn write_float_array(a : &RustFloatArray, r : &mut Vec<KVal>){
    let vec = a.vec();
    r.push(comp_int(vec.len() as i64));
    for f in vec{
        r.push(comp_double(*f));
    }
}