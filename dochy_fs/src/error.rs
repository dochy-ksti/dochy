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

impl From<dochy_archiver2::NouArcError> for FsError{
    fn from(e : dochy_archiver2::NouArcError) -> Self{ Self::new(e) }
}

impl From<dochy_core::CoreError> for FsError{
    fn from(e : dochy_core::CoreError) -> Self{ Self::new(e) }
}

impl From<dochy_diff::DiffError> for FsError{
    fn from(e : dochy_diff::DiffError) -> Self{ Self::new(e) }
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

//impl From<std::sys_common::poison::PoisonError<Guard>> for FsError{
  //  fn from(e : dochy_diff::diff_error::DiffError) -> Self{ Self::new(e) }
//}