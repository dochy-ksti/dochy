#![feature(test)]
#![feature(try_trait)]
extern crate test;

mod imp;
#[cfg(test)]
#[allow(dead_code, unused_imports)]
mod testing;
pub mod error;



pub type ArcResult<T> = std::result::Result<T, crate::error::NouArcError>;

pub use imp::create_archive_from_directory::create_archive_from_directory;
pub use imp::read_archive_data::read_archive_data;
pub use imp::get_hash_and_metadata_from_dir::get_hash_and_metadata_from_dir;
pub use imp::get_hash_and_metadata_from_archive::get_hash_and_metadata_from_archive;

pub use imp::resume_archive::resume_archive;

pub use imp::structs::metadata::Metadata;
pub use imp::structs::metadata_item::MetadataItem;
pub use imp::create_archive_from_directory::CreateArchiveFromDirectory;
pub use imp::structs::archive_data::ArchiveData;


pub use imp::structs::archive_options::ArchiveOptions;
pub use imp::structs::archive_options::ArchiveOptionsBuilder;