#![feature(try_trait)]
#![feature(vec_into_raw_parts)]
#![feature(test)]
extern crate test;

#[cfg(test)]
mod sample_test;

pub mod json5;
pub mod compaction;
pub mod intf;
pub mod archiver;
pub mod core;
pub mod diff;
pub mod fs;
