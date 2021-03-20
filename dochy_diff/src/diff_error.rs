use std::option::NoneError;
use anyhow::anyhow;
use std::fmt::{Debug, Display, Formatter};
use with_capacity_safe::WcsError;

pub struct DiffError {
    e : anyhow::Error
}
impl DiffError{
    pub fn new(e : impl Into<anyhow::Error>) -> Self{ Self{ e : e.into() } }
}

impl Display for DiffError{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.e, f)
    }
}

impl Debug for DiffError{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.e, f)
    }
}

impl Into<anyhow::Error> for DiffError {
    fn into(self) -> anyhow::Error { self.e }
}

impl From<NoneError> for DiffError{
    fn from(_: NoneError) -> Self {
        DiffError::new(anyhow!("None Error"))
    }
}
impl From<anyhow::Error> for DiffError{
    fn from(e: anyhow::Error) -> Self {
        Self::new(e)
    }
}

impl From<WcsError> for DiffError{
    fn from(e: WcsError) -> Self {
        DiffError::new(e)
    }
}

impl From<&str> for DiffError{
    fn from(e: &str) -> Self {
        Self::new(anyhow!("{}", e))
    }
}

impl From<String> for DiffError{
    fn from(e: String) -> Self {
        Self::new(anyhow!("{}", e))
    }
}

