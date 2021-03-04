#![feature(try_trait)]
#![feature(vec_into_raw_parts)]
#![feature(test)]
extern crate test;

#[cfg(test)]
mod sample_test;

pub mod archiver;
pub mod compaction;
pub mod core;
pub mod diff;
pub mod fs;
pub mod intf;
pub mod json5;
