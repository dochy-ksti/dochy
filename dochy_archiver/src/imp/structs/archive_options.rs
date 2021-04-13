use std::collections::HashSet;
use crate::ArcResult;

/// Customize how we build the archive.
///
/// If you need the default value, you can get it with new()
/// ```
/// use dochy_archiver::ArchiveOptions;
///
/// // construct with the default values.
/// // In the default configuration, every file is archived in the folder and the subfolders.
/// let op = ArchiveOptions::new();
/// ```
/// If you want to customize, you can use the builder.
///
/// Appending "..Default::default()" could make your source code
/// compatible with the future versions of this library.
/// ```
/// use dochy_archiver::ArcResult;
/// use dochy_archiver::{ArchiveOptions, ArchiveOptionsBuilder};
///
/// fn main() -> ArcResult<()>{
/// let op = ArchiveOptions::from(
///         ArchiveOptionsBuilder {
///             extensions_archived : vec![".json5"],
///             archive_subfolders : false,
///             ..Default::default()
///        })?;
///     Ok(())
/// }
/// ```
#[derive(Debug, Clone)]
pub struct ArchiveOptions {
    extensions_archived : HashSet<String>,
    archive_subfolders : bool,
}

impl ArchiveOptions {

    /// If the file with the extension is archived?
    pub fn is_archived(&self, extension : &str) -> bool{
        if self.extensions_archived.len() == 0{
            true
        } else{
            self.extensions_archived.contains(extension)
        }
    }

    /// If the subfolders are archived
    pub fn archive_subfolders(&self) -> bool{ self.archive_subfolders }

    /// construct with the default values.
    /// In the default configuration, every file is archived in the folder and the subfolders.
    pub fn new() -> ArchiveOptions {
        Self::from(Default::default()).unwrap()
    }

    /// Construct ArchiveOptions from the builder
    pub fn from(builder : ArchiveOptionsBuilder) -> ArcResult<ArchiveOptions>{
        let extensions_archived : HashSet<String> = builder
            .extensions_archived.into_iter()
            .map(|s| s.to_string()).collect();

        let archive_subfolders = builder.archive_subfolders;
        Ok(ArchiveOptions {
            extensions_archived,
            archive_subfolders,
        })
    }
}

///Construct ArchiveOptions
#[derive(Debug, Clone)]
pub struct ArchiveOptionsBuilder<'a> {
    pub extensions_archived : Vec<&'a str>,
    pub archive_subfolders : bool,
}

impl<'a> Default for ArchiveOptionsBuilder<'a> {
    fn default() -> Self {
        Self{
            extensions_archived : vec![],
            archive_subfolders : true,
        }
    }
}
