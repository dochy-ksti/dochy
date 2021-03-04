use std::fmt::{Display, Formatter, Debug};
//use std::backtrace::Backtrace;
use std::option::NoneError;
use anyhow::{anyhow};
use std::time::SystemTimeError;
use with_capacity_safe::WcsError;
//use std::time::SystemTimeError;

pub type FsResult<T> = Result<T, FsError>;

pub struct FsError {
    error : anyhow::Error,
}

impl FsError {
    pub fn new(e : impl Into<anyhow::Error>) -> Self{ Self{ error : e.into() } }
}

impl Display for FsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.error, f)
    }
}

impl Debug for FsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.error, f)
    }
}

impl Into<anyhow::Error> for FsError {
    fn into(self) -> anyhow::Error {
        self.error
    }
}

impl From<NoneError> for FsError {
    fn from(_: NoneError) -> Self {
        FsError::new(anyhow!("None Error"))
    }
}
impl From<anyhow::Error> for FsError {
    fn from(e: anyhow::Error) -> Self {
        Self::new(e)
    }
}

impl From<SystemTimeError> for FsError {
    fn from(e : SystemTimeError) -> Self { Self::new(e) }
}

impl From<std::io::Error> for FsError {
    fn from(e : std::io::Error) -> Self { Self::new(e) }
}

impl From<crate::archiver::error::NouArcError> for FsError{
    fn from(e : crate::archiver::error::NouArcError) -> Self{ Self::new(e) }
}

impl From<crate::core::error::CoreError> for FsError{
    fn from(e : crate::core::error::CoreError) -> Self{ Self::new(e) }
}

impl From<crate::diff::diff_error::DiffError> for FsError{
    fn from(e : crate::diff::diff_error::DiffError) -> Self{ Self::new(e) }
}

impl From<WcsError> for FsError{
    fn from(e : WcsError) -> Self{ Self::new(e) }
}

impl From<&str> for FsError{
    fn from(e : &str) -> Self{ Self::new(anyhow!("{}", e)) }
}

impl From<String> for FsError{
    fn from(e : String) -> Self{ Self::new(anyhow!("{}", e)) }
}
