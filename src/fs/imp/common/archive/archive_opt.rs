use crate::archiver::{ArchiveOptions, ArchiveOptionsBuilder};
use once_cell::sync::Lazy;

pub static JSON_ARC_OPT : Lazy<ArchiveOptions> = Lazy::new(||{
    ArchiveOptions::from(ArchiveOptionsBuilder{
        extensions_archived : vec!["json5"],
        archive_subfolders : false,
    }).unwrap()
});
