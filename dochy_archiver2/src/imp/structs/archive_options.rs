use std::collections::HashSet;
use crate::ArcResult;


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
