#![feature(try_trait)]
#![feature(test)]
#![feature(vec_into_raw_parts)]
extern crate test;

#[cfg(test)]
#[allow(dead_code)]
mod test_diff;

mod imp;
pub mod diff_error;

pub use imp::get_diff::get_diff;
pub use imp::get_diff::get_kvals;
pub use imp::apply_diff::apply_diff;
