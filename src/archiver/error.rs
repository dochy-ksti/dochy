use std::fmt::{Display, Formatter, Debug};
//use std::backtrace::Backtrace;
use std::option::NoneError;
use anyhow::{anyhow};
use std::time::SystemTimeError;
use with_capacity_safe::WcsError;
use std::path::StripPrefixError;

/// The error type.
///
/// This wraps anyhow::Error. You can get it from Into trait.
///
/// Maybe the source error retrieved from anyhow::Error can be used to determine the cause of the error,
/// but currently, there's no guarantees about the error format.
///
/// I deeply depend on anyhow::Error::backtrace for debugging.
pub struct NouArcError{
    error : anyhow::Error,
}

impl NouArcError{
    pub(crate) fn new(e : impl Into<anyhow::Error>) -> Self{ Self{ error : e.into() } }
}

impl Display for NouArcError{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.error, f)
    }
}

impl Debug for NouArcError{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.error, f)
    }
}

impl Into<anyhow::Error> for NouArcError{
    fn into(self) -> anyhow::Error {
        self.error
    }
}

impl From<NoneError> for NouArcError{
    fn from(_: NoneError) -> Self {
        NouArcError::new(anyhow!("None Error"))
    }
}


impl From<anyhow::Error> for NouArcError{
    fn from(e: anyhow::Error) -> Self {
        Self::new(e)
    }
}

impl From<SystemTimeError> for NouArcError{
    fn from(e : SystemTimeError) -> Self { Self::new(e) }
}

impl From<StripPrefixError> for NouArcError{
    fn from(e: StripPrefixError) -> Self {
        NouArcError::new(e)
    }
}


impl From<std::io::Error> for NouArcError{
    fn from(e : std::io::Error) -> Self { Self::new(e) }
}

impl From<WcsError> for NouArcError{
    fn from(e : WcsError) -> Self { Self::new(e) }
}

impl From<std::string::FromUtf8Error> for NouArcError{
    fn from(e : std::string::FromUtf8Error) -> Self { Self::new(e) }
}