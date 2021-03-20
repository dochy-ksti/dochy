use dochy_core::structs::{MetaParam, VarType, RustParam, ParamType, Qv, RustString, RustIntArray, RustFloatArray};
use crate::diff_error::DiffError;
use crate::imp::read::reader::Reader;
use crate::imp::read::get_null::get_null;
use crate::imp::read::get_undefined::get_undefined;
use with_capacity_safe::vec_with_capacity_safe;

pub(crate) fn read_param(meta : &MetaParam, r : &mut Reader) -> Result<RustParam, DiffError>{
    match meta.var_type() {
        VarType::Normal => {
            read_param2(meta, r)
        },
        VarType::Nullable => {
            if r.read()?.as_bool()?{
                read_param2(meta, r)
            } else{
                Ok(get_null(meta.param_type()))
            }
        },
        VarType::Undefiable =>{
            if r.read()?.as_bool()?{
                read_param2(meta, r)
            } else{
                Ok(get_undefined(meta.param_type()))
            }
        },
        VarType::UndefNullable =>{
            if r.read()?.as_bool()?{
                read_param2(meta, r)
            } else if r.read()?.as_bool()?{
                Ok(get_null(meta.param_type()))
            } else{
                Ok(get_undefined(meta.param_type()))
            }
        }
    }
}

fn read_param2(meta : &MetaParam, r : &mut Reader) -> Result<RustParam, DiffError> {
    let p = match meta.param_type() {
        ParamType::Bool => { RustParam::Bool(Qv::Val(r.read()?.as_bool()?)) }
        ParamType::Int => { RustParam::Int(Qv::Val(r.read()?.as_i64()?)) }
        ParamType::Float => { RustParam::Float(Qv::Val(r.read()?.as_f64()?)) }
        ParamType::String => {
            RustParam::String(
                Qv::Val(RustString::new(r.read()?.as_string()?)))
        }
        ParamType::IntArray => { RustParam::IntArray(Qv::Val(read_int_array(r)?)) }
        ParamType::FloatArray => { RustParam::FloatArray(Qv::Val(read_float_array(r)?)) }
    };
    Ok(p)
}

fn read_int_array(r : &mut Reader) -> Result<RustIntArray, DiffError>{
    let len = r.read()?.as_i64()? as usize;
    let mut vec : Vec<i64> = vec_with_capacity_safe(len)?;
    for _ in 0..len{
        vec.push(r.read()?.as_i64()?)
    }
    Ok(RustIntArray::new(vec))
}

fn read_float_array(r : &mut Reader) -> Result<RustFloatArray, DiffError>{
    let len = r.read()?.as_i64()? as usize;
    let mut vec : Vec<f64> = vec_with_capacity_safe(len)?;
    for _ in 0..len{
        vec.push(r.read()?.as_f64()?)
    }
    Ok(RustFloatArray::new(vec))
}