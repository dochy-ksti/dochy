use std::fmt::{Display, Formatter, Debug};
//use std::backtrace::Backtrace;
use std::option::NoneError;
use anyhow::{anyhow};
use std::time::SystemTimeError;
//use std::time::SystemTimeError;

pub type DpResult<T> = Result<T, DpError>;

pub struct DpError {
    error : anyhow::Error,
}

impl DpError {
    pub fn new(e : impl Into<anyhow::Error>) -> Self{ Self{ error : e.into() } }
}

impl Display for DpError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.error, f)
    }
}

impl Debug for DpError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.error, f)
    }
}

impl Into<anyhow::Error> for DpError {
    fn into(self) -> anyhow::Error {
        self.error
    }
}

impl From<NoneError> for DpError {
    fn from(_: NoneError) -> Self {
        DpError::new(anyhow!("None Error"))
    }
}
impl From<anyhow::Error> for DpError {
    fn from(e: anyhow::Error) -> Self {
        Self::new(e)
    }
}

impl From<SystemTimeError> for DpError {
    fn from(e : SystemTimeError) -> Self { Self::new(e) }
}

impl From<std::io::Error> for DpError {
    fn from(e : std::io::Error) -> Self { Self::new(e) }
}

impl From<crate::archiver::error::NouArcError> for DpError{
    fn from(e : crate::archiver::error::NouArcError) -> Self{ Self::new(e) }
}

impl From<crate::core::error::CoreError> for DpError{
    fn from(e : crate::core::error::CoreError) -> Self{ Self::new(e) }
}

impl From<crate::diff::diff_error::DiffError> for DpError{
    fn from(e : crate::diff::diff_error::DiffError) -> Self{ Self::new(e) }
}


impl From<&str> for DpError{
    fn from(e : &str) -> Self{ Self::new(anyhow!("{}", e)) }
}

impl From<String> for DpError{
    fn from(e : String) -> Self{ Self::new(anyhow!("{}", e)) }
}
