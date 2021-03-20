use crate::imp::structs::qv::Qv;
use std::ptr::null;

/// &参照を露出してしまうと、それが生きている間にwriteしたら当然UB(undefined behavior)になる。
/// 参照を見せないために、RustではStringのCopyを行う
/// 対してCからは、ポインタを介してアクセスする分にはUBにならないのでコピーしない(そもそもコピーしても破棄するのが大変・・・
///
/// 生ポインタを持つ構造体はSendでもSyncでもないので、マルチスレッドで使われる心配はしなくていい。この方式ではマルチスレッド対応は無理
/// (まあ使う側がreadとwriteが絶対にかぶらないようにしてくれればいいのだが)
///
/// MutItemが削除されるとポインタは不正になる
/// このポインタがdefaultを指していると、Stringがsabunにsetされた場合、defaultの情報を指し続けることになり、不整合になる。
/// String自体は生きていても、stringのcapacity変更に伴う再構築が起きれば、ptrから取った*const u8は無効になる
///
/// まあ一般的に、書き込んだ後にはこのポインタは無効になると考えるべき。
#[repr(C)] #[derive(Debug, Clone, Copy)]
pub struct StrPtr {
    s : *const String,
}
impl StrPtr {
    pub fn new(s : *const String) -> StrPtr { StrPtr { s } }
    pub fn to_string(&self) -> String{
        let s = unsafe{&*self.s };
        s.to_string()
    }
}

#[no_mangle] #[allow(non_snake_case)]
pub extern "C" fn StrPtr_Len(p : StrPtr) -> u64{
    let s = unsafe{&*p.s};
    s.len() as u64
}
#[no_mangle] #[allow(non_snake_case)]
pub extern "C" fn StrPtr_Ptr(p : StrPtr) -> *const u8{
    let s = unsafe{&*p.s};
    s.as_ptr()
}
#[no_mangle] #[allow(non_snake_case)]
pub extern "C" fn StrPtr_IsNull(p : StrPtr) -> i8{
    p.s.is_null() as i8
}

#[repr(C)] #[derive(Debug, Clone, Copy)]
pub struct QvStrPtr{
    s : *const Qv<String>,
}
impl QvStrPtr{
    pub fn new(s : *const Qv<String>)-> QvStrPtr{ QvStrPtr{ s } }
    pub fn value(&self) -> Qv<String>{
        let qv = unsafe{ &*self.s };
        qv.clone()
    }
}
#[no_mangle] #[allow(non_snake_case)]
pub extern "C" fn QvStrPtr_NullOr(p : QvStrPtr) -> StrPtr {
    let p = unsafe{ &*p.s };
    match p{
        Qv::Val(s) => StrPtr::new(s),
        Qv::Null => StrPtr::new(null()),
        _ => unreachable!(),
    }
}
#[no_mangle] #[allow(non_snake_case)]
pub extern "C" fn QvStrPtr_UndefOr(p : QvStrPtr) -> StrPtr {
    let p = unsafe{ &*p.s };
    match p{
        Qv::Val(s) => StrPtr::new(s),
        Qv::Undefined => StrPtr::new(null()),
        _ => unreachable!(),
    }
}
#[no_mangle] #[allow(non_snake_case)]
pub extern "C" fn QvStrPtr_QvValue(p : QvStrPtr) -> StrPtr {
    let p = unsafe{ &*p.s };
    match p{
        Qv::Val(s) => StrPtr::new(s),
        _ => unreachable!(),
    }
}
#[no_mangle] #[allow(non_snake_case)]
pub extern "C" fn QvStrPtr_IsNull(p : QvStrPtr) -> i8{
    let p = unsafe{ &*p.s };
    match p{
        Qv::Null => 1,
        _ => 0,
    }
}
#[no_mangle] #[allow(non_snake_case)]
pub extern "C" fn QvStrPtr_IsUndefined(p : QvStrPtr) -> i8{
    let p = unsafe{ &*p.s };
    match p{
        Qv::Undefined => 1,
        _ => 0,
    }
}