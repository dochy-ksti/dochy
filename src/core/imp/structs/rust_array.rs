use crate::core::imp::structs::qv::Qv;
use crate::core::imp::structs::rust_param::RustParam;
use crate::core::imp::structs::array_type::ArrayType;

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

    pub(crate) fn qv(&self) -> &Qv<Vec<RustParam>>{ self.array.as_ref() }

    pub(crate) fn to_float_array(&self) -> Option<Qv<RustFloatArray>>{
        self.qv().opt_map(|a| RustFloatArray::from_params(a))
    }

    pub(crate) fn to_int_array(&self) -> Option<Qv<RustIntArray>>{
        self.qv().opt_map(|a| RustIntArray::from_params(a))
    }

    pub(crate) fn to_param(&self, at : &ArrayType) -> Option<RustParam>{
        Some(match at{
            ArrayType::Float =>{ RustParam::FloatArray(self.to_float_array()?) },
            ArrayType::Int =>{ RustParam::IntArray(self.to_int_array()?) },
            //ArrayType::String =>{ RustParam::StrArray(self.to_str_array()?)}
            //ArrayType::Num2 =>{ RustParam::Num2Array(self.to_num2_array()?)}
        })
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct RustFloatArray{
    b : Box<Vec<f64>>,
}

impl RustFloatArray{
    pub fn new(b : Vec<f64>) -> RustFloatArray{ RustFloatArray{ b : Box::new(b) }}
    //pub(crate) fn as_ref(&self) -> &Vec<f64>{ self.b.as_ref() }
    pub(crate) fn to_params(&self) -> Vec<RustParam>{
        self.b.iter().map(|a| RustParam::Float(Qv::Val(*a))).collect()
    }
    pub(crate) fn from_params(v : &Vec<RustParam>) -> Option<RustFloatArray>{
        let op  = v.iter().map(|p| p.to_float()).collect::<Option<Vec<f64>>>();
        Some(RustFloatArray::new(op?))
    }
    pub fn vec(&self) -> &Vec<f64>{ self.b.as_ref() }
}

#[derive(Debug, PartialEq, Clone)]
pub struct RustIntArray{
    b : Box<Vec<i64>>,
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


