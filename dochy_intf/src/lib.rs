#[allow(dead_code)]
mod test_generated;

#[allow(dead_code)]
mod test;
mod imp;
mod util;

pub use imp::generate_interface::generate_interface;

pub use imp::structs::root_source::RootSource;
