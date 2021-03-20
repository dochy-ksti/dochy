use crate::imp::structs::qv::Qv;


#[repr(C)] #[derive(Debug, Clone, Copy)]
pub struct QvInt{
    val : i64,
    is_null : u32,
    is_undefined : u32,
}
impl QvInt{
    pub fn new(s : Qv<i64>)-> QvInt{
        match s{
            Qv::Val(i) => QvInt{ val : i, is_null : 0, is_undefined : 0 },
            Qv::Null => QvInt{ val : 0, is_null : 1, is_undefined : 0 },
            Qv::Undefined => QvInt{ val : 0, is_null : 0, is_undefined : 1 },
        }
    }
}
#[repr(C)] #[derive(Debug, Clone, Copy)]
pub struct OptInt{
    val : i64,
    is_valid : u32,
}
impl OptInt{
    pub fn new(val : i64, is_valid : u32) -> OptInt{ OptInt{ val, is_valid } }
    pub fn null_or(qv : Qv<i64>) -> OptInt{
        match qv{
            Qv::Val(i) => OptInt::new(i, 1),
            //一応奇妙で微妙な違いを出しておく
            Qv::Null => OptInt::new(0, 0),
            _ => OptInt::new(1, 0),
        }
    }
    pub fn undef_or(qv : Qv<i64>) -> OptInt{
        match qv{
            Qv::Val(i) => OptInt::new(i, 1),
            Qv::Undefined => OptInt::new(0,0),
            _ => OptInt::new(1, 0),
        }
    }
}

#[no_mangle] #[allow(non_snake_case)]
pub extern "C" fn QvInt_Value(p : QvInt) -> i64{
    p.val
}
#[no_mangle] #[allow(non_snake_case)]
pub extern "C" fn QvInt_IsNull(p : QvInt) -> i8{
    p.is_null as i8
}
#[no_mangle] #[allow(non_snake_case)]
pub extern "C" fn QvInt_IsUndefined(p : QvInt) -> i8{
    p.is_undefined as i8
}
#[no_mangle] #[allow(non_snake_case)]
pub extern "C" fn OptInt_Value(p : OptInt) -> i64{
    p.val
}
#[no_mangle] #[allow(non_snake_case)]
pub extern "C" fn OptInt_IsValid(p : OptInt) -> i8{
    p.is_valid as i8
}