use crate::imp::structs::qv::Qv;


#[repr(C)] #[derive(Debug, Clone, Copy)]
pub struct QvBool{
    val : i8,
    is_null : i8,
    is_undefined : i8,
}
impl QvBool{
    pub fn new(s : Qv<bool>)-> QvBool {
        match s{
            Qv::Val(b) => QvBool{ val : b as i8, is_null : 0, is_undefined : 0 },
            Qv::Null => QvBool{ val : 0, is_null : 1, is_undefined : 0 },
            Qv::Undefined => QvBool{ val : 0, is_null : 0, is_undefined : 1 },
        }
    }
}
#[repr(C)] #[derive(Debug, Clone, Copy)]
pub struct OptBool{
    val : i8,
    is_valid : i8,
}
impl OptBool{
    pub fn new(val : i8, is_valid : i8) -> OptBool{ OptBool{ val, is_valid } }
    pub fn null_or(qv : Qv<i8>) -> OptBool{
        match qv{
            Qv::Val(i) => OptBool::new(i, 1),
            //一応奇妙で微妙な違いを出しておく
            Qv::Null => OptBool::new(0, 0),
            _ => OptBool::new(1, 0),
        }
    }
    pub fn undef_or(qv : Qv<i8>) -> OptBool{
        match qv{
            Qv::Val(i) => OptBool::new(i, 1),
            Qv::Undefined => OptBool::new(0,0),
            _ => OptBool::new(1, 0),
        }
    }
}

#[no_mangle] #[allow(non_snake_case)]
pub extern "C" fn QvBool_Value(p : QvBool) -> i8{
    p.val
}
#[no_mangle] #[allow(non_snake_case)]
pub extern "C" fn QvBool_IsNull(p : QvBool) -> i8{
    p.is_null as i8
}
#[no_mangle] #[allow(non_snake_case)]
pub extern "C" fn QvBool_IsUndefined(p : QvBool) -> i8{
    p.is_undefined as i8
}
#[no_mangle] #[allow(non_snake_case)]
pub extern "C" fn OptBool_Value(p : OptBool) -> i8{
    p.val
}
#[no_mangle] #[allow(non_snake_case)]
pub extern "C" fn OptBool_IsValid(p : OptBool) -> i8{
    p.is_valid as i8
}