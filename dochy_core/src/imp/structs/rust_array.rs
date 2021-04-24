use crate::imp::structs::qv::Qv;
use crate::imp::structs::rust_param::RustParam;
use crate::imp::structs::array_type::ArrayType;
use crate::error::CoreResult;
use crate::imp::structs::rust_identity::RustIdentity;

#[derive(Debug, PartialEq, Clone)]
pub struct RustArray{
    array : Box<Qv<Vec<RustParam>>>,
}

impl RustArray{
    pub(crate) fn new(qv : Qv<Vec<RustParam>>) -> RustArray{
        RustArray{ array : Box::new(qv) }
    }

    pub(crate) fn from_int_array(qv : &Qv<RustIntArray>) -> RustArray{
        RustArray::new(qv.map(|a| a.to_params()))
    }

    pub(crate) fn from_float_array(qv : &Qv<RustFloatArray>) -> RustArray{
        RustArray::new(qv.map(|a| a.to_params()))
    }

    pub(crate) fn from_binary(qv : &Qv<RustBinary>) -> RustArray{
        RustArray::new(qv.map(|a| a.to_params()))
    }

    pub(crate) fn qv(&self) -> &Qv<Vec<RustParam>>{ self.array.as_ref() }

    pub(crate) fn to_float_array(&self) -> Option<Qv<RustFloatArray>>{
        self.qv().opt_map(|a| RustFloatArray::from_params(a))
    }

    pub(crate) fn to_int_array(&self) -> Option<Qv<RustIntArray>>{
        self.qv().opt_map(|a| RustIntArray::from_params(a))
    }

    pub(crate) fn to_binary(&self) -> Option<Qv<RustBinary>>{
        self.qv().opt_map(|a| RustBinary::from_params(a))
    }

    pub(crate) fn to_param(&self, at : &ArrayType) -> CoreResult<RustParam>{
        Ok(match at{
            ArrayType::Float =>{ RustParam::FloatArray(self.to_float_array().ok_or("FloatArray is not valid")?) },
            ArrayType::Int =>{ RustParam::IntArray(self.to_int_array().ok_or("IntArray is not valid")?) },
            ArrayType::Binary =>{ RustParam::Binary(self.to_binary().ok_or("Binary is not valid")?) },
            //ArrayType::String =>{ RustParam::StrArray(self.to_str_array()?)}
            //ArrayType::Num2 =>{ RustParam::Num2Array(self.to_num2_array()?)}
        })
    }
}

#[derive(Debug, Clone)]
pub struct RustFloatArray{
    b : Box<(Vec<f64>, RustIdentity)>,
}

impl RustFloatArray{
    pub fn new(b : Vec<f64>) -> RustFloatArray{ RustFloatArray{ b : Box::new((b, RustIdentity::new())) }}
    //pub(crate) fn as_ref(&self) -> &Vec<f64>{ self.b.as_ref() }
    pub(crate) fn to_params(&self) -> Vec<RustParam>{
        self.vec().iter().map(|a| RustParam::Float(Qv::Val(*a))).collect()
    }
    pub(crate) fn from_params(v : &Vec<RustParam>) -> Option<RustFloatArray>{
        let op  = v.iter().map(|p| p.to_float()).collect::<Option<Vec<f64>>>();
        Some(RustFloatArray::new(op?))
    }
    pub fn vec(&self) -> &Vec<f64>{ &self.b.as_ref().0 }
}

#[derive(Debug, Clone)]
pub struct RustIntArray{
    b : Box<(Vec<i64>, RustIdentity)>,
}

impl RustIntArray{
    pub fn new(b : Vec<i64>) -> RustIntArray{ RustIntArray{ b : Box::new(b) }}
    //pub(crate) fn as_ref(&self) -> &Vec<i64>{ self.b.as_ref() }
    pub(crate) fn to_params(&self) -> Vec<RustParam>{
        self.b.iter().map(|a| RustParam::Int(Qv::Val(*a))).collect()
    }
    pub(crate) fn from_params(v : &Vec<RustParam>) -> Option<RustIntArray>{
        let op  = v.iter().map(|p| p.to_int()).collect::<Option<Vec<i64>>>();
        Some(RustIntArray::new(op?))
    }
    pub fn vec(&self) -> &Vec<i64>{ self.b.as_ref() }
}

#[derive(Debug, Clone)]
pub struct RustBinary{
    b : Box<(Vec<u8>, RustIdentity)>,
}

impl RustBinary{
    pub fn new(b : Vec<u8>) -> RustBinary{ RustBinary{ b : Box::new((b, RustIdentity)) }}
    pub(crate) fn to_params(&self) -> Vec<RustParam>{
        self.b.iter().map(|a| RustParam::Int(Qv::Val(*a as i64))).collect()
    }
    pub(crate) fn from_params(v : &Vec<RustParam>) -> Option<RustBinary>{
        let op  = v.iter().map(|p| p.to_u8()).collect::<Option<Vec<u8>>>();
        Some(RustBinary::new(op?))
    }
    pub fn vec(&self) -> &Vec<u8>{ &self.b.as_ref().0 }
    pub fn vec_mut(&mut self) -> &mut Vec<u8>{ self.b.as_mut() }
}