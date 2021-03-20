use crate::imp::structs::qv::Qv;
#[derive(Debug, PartialEq)]
pub enum NullOr<T>{
    Val(T),
    Null
}
#[derive(Debug, PartialEq)]
pub enum UndefOr<T>{
    Val(T),
    Undefined,
}

impl<T : Clone> Clone for NullOr<T>{
    fn clone(&self) -> Self {
        self.map(|t| t.clone())
    }
}

impl<T : Clone> Clone for UndefOr<T>{
    fn clone(&self) -> Self {
        self.map(|t| t.clone())
    }
}

impl<T> NullOr<T>{
    pub fn map<U>(&self, f : impl Fn(&T) -> U) -> NullOr<U> {
        match self {
            NullOr::Val(v) => NullOr::Val(f(v)),
            NullOr::Null => NullOr::Null,
        }
    }

    pub fn opt_map<U>(&self, f : impl Fn(&T) -> Option<U>) -> Option<NullOr<U>>{
        match self {
            NullOr::Val(v) => f(v).map(|r| NullOr::Val(r)),
            NullOr::Null => Some(NullOr::Null),
        }
    }

    pub fn from_qv(v : Qv<T>) -> Option<NullOr<T>>{
        match v{
            Qv::Val(v) => Some(NullOr::Val(v)),
            Qv::Null => Some(NullOr::Null),
            Qv::Undefined => None,
        }
    }

    pub fn into_qv(self) -> Qv<T>{
        match self{
            NullOr::Val(v) => Qv::Val(v),
            NullOr::Null => Qv::Null,
        }
    }
}



impl<T> UndefOr<T>{
    pub fn map<U>(&self, f : impl Fn(&T) -> U) -> UndefOr<U> {
        match self {
            UndefOr::Val(v) => UndefOr::Val(f(v)),
            UndefOr::Undefined => UndefOr::Undefined,
        }
    }

    pub fn opt_map<U>(&self, f : impl Fn(&T) -> Option<U>) -> Option<UndefOr<U>>{
        match self {
            UndefOr::Val(v) => f(v).map(|r| UndefOr::Val(r)),
            UndefOr::Undefined => Some(UndefOr::Undefined),
        }
    }

    pub fn from_qv(v : Qv<T>) -> Option<UndefOr<T>>{
        match v{
            Qv::Val(v) => Some(UndefOr::Val(v)),
            Qv::Undefined => Some(UndefOr::Undefined),
            Qv::Null => None,
        }
    }

    pub fn into_qv(self) -> Qv<T>{
        match self{
            UndefOr::Val(v) => Qv::Val(v),
            UndefOr::Undefined => Qv::Undefined,
        }
    }
}