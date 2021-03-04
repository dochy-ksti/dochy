use crate::archiver::ArchiveData;
use crate::core::structs::{RootObject};
use crate::core::{json_files_to_rust, JsonFile};
use crate::fs::error::FsResult;
use std::str::from_utf8;
use std::path::Path;

pub fn read_archive(archive : &ArchiveData, validation : bool) -> FsResult<RootObject>{
    Ok(json_files_to_rust(archive.iter().map(|(path, bytes)| to_json_file(path, bytes)) , validation)?)
}

struct JsonFileImpl<'a>{
    contents : Str<'a>,
    filename_without_ext : String
}

//Cow使えばいい説はある
enum Str<'a>{
    Ref(&'a str),
    String(String)
}

impl<'a> JsonFile for JsonFileImpl<'a>{
    fn filename_without_ext(&self) -> &str {
        &self.filename_without_ext
    }

    fn json(&self) -> &str {
        match &self.contents{
            Str::Ref(s) => *s,
            Str::String(s) => s,
        }
    }
}

fn to_json_file<'a, 'b>(path : &'a str, bytes : &'b [u8]) -> JsonFileImpl<'b>{
    let path = Path::new(path);
    let filename_without_ext = path.file_stem().map(|stem| stem.to_string_lossy().to_string()).unwrap_or_else(||"unknownname".to_string());
    let contents = match from_utf8(bytes){
        Ok(s) => Str::Ref(s),
        Err(_e) => Str::String(String::from_utf8_lossy(bytes).to_string())
    };
    JsonFileImpl{ contents, filename_without_ext }
}