use crate::imp::structs::qv::Qv;


#[repr(C)] #[derive(Debug, Clone, Copy)]
pub struct QvFloat{
    val : f64,
    is_null : u32,
    is_undefined : u32,
}
impl QvFloat{
    pub fn new(s : Qv<f64>)-> QvFloat{
        match s{
            Qv::Val(f) => QvFloat{ val : f, is_null : 0, is_undefined : 0 },
            Qv::Null => QvFloat{ val : 0.0, is_null : 1, is_undefined : 0 },
            Qv::Undefined => QvFloat{ val : 0.0, is_null : 0, is_undefined : 1 },
        }
    }
}
#[repr(C)] #[derive(Debug, Clone, Copy)]
pub struct OptFloat{
    val : f64,
    is_valid : u32,
}
impl OptFloat{
    pub fn new(val : f64, is_valid : u32) -> OptFloat{ OptFloat{ val, is_valid } }
    pub fn null_or(qv : Qv<f64>) -> OptFloat{
        match qv{
            Qv::Val(i) => OptFloat::new(i, 1),
            //一応奇妙で微妙な違いを出しておく
            Qv::Null => OptFloat::new(0.0, 0),
            _ => OptFloat::new(1.0, 0),
        }
    }
    pub fn undef_or(qv : Qv<f64>) -> OptFloat{
        match qv{
            Qv::Val(i) => OptFloat::new(i, 1),
            Qv::Undefined => OptFloat::new(0.0,0),
            _ => OptFloat::new(1.0, 0),
        }
    }
}

#[no_mangle] #[allow(non_snake_case)]
pub extern "C" fn QvFloat_Value(p : QvFloat) -> f64{
    p.val
}
#[no_mangle] #[allow(non_snake_case)]
pub extern "C" fn QvFloat_IsNull(p : QvFloat) -> i8{
    p.is_null as i8
}
#[no_mangle] #[allow(non_snake_case)]
pub extern "C" fn QvFloat_IsUndefined(p : QvFloat) -> i8{
    p.is_undefined as i8
}
#[no_mangle] #[allow(non_snake_case)]
pub extern "C" fn OptFloat_Value(p : OptFloat) -> f64{
    p.val
}
#[no_mangle] #[allow(non_snake_case)]
pub extern "C" fn OptFloat_IsValid(p : OptFloat) -> i8{
    p.is_valid as i8
}