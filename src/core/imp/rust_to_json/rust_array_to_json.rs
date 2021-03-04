use crate::core::imp::rust_to_json::get_param::get_param;
use crate::core::imp::structs::array_type::ArrayType;
use crate::core::imp::structs::qv::Qv;
use crate::core::imp::structs::my_json::Value;
use crate::core::imp::structs::rust_array::RustArray;

pub(crate) fn rust_array_to_json(array : &RustArray, at : &ArrayType) -> Value{
    let mut result : Vec<Value> = vec![];


    match at{
        //ArrayType::String =>{ result.push(Value::String("StrArray".to_string())) },
        ArrayType::Int =>{
            let array_len = if let Qv::Val(v) = array.qv(){
                v.len()
            } else{ 0 };
            //noisyすぎるので基本省略
            if array_len == 0 {
                result.push(Value::String("IntArray".to_string()))
            }
        },
        ArrayType::Float => {
            result.push(Value::String("FloatArray".to_string()))
        },
        //ArrayType::Num2 =>{ result.push(Value::String("Num2Array".to_string())) }
    }
    match array.qv(){
        Qv::Val(v) => {
            for item in v{
                result.push(get_param(item));
            }
        },
        Qv::Undefined =>{ result.push(Value::Undefined) },
        Qv::Null =>{ result.push(Value::Null) },
    }
    return Value::Array(result);
}