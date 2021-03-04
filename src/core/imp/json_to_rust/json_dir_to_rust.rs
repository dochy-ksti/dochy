
use std::fs::File;

use std::io::prelude::*;
use crate::core::error::CoreResult;
use std::ffi::{OsStr};
use crate::core::imp::json_to_rust::{json_root_to_rust, json_item_str_to_rust};
use crate::core::{HashM, HashMt};

use crate::core::imp::json_to_rust::construct_root::construct_root;
use crate::core::imp::structs::root_obj::RootObject;
use crate::core::imp::structs::json_file::{JsonFile, JsonFileImpl};
use crate::core::imp::structs::root_value::RootValue;
use std::path::Path;

pub fn json_dir_to_rust<P : AsRef<Path>>(dir_path : P, validation : bool) -> CoreResult<RootObject>{
    let dirs = std::fs::read_dir(dir_path)?;

    let mut vec : Vec<JsonFileImpl> = vec![];

    for dir in dirs{
        match dir{
            Ok(de) =>{
                let path = de.path();
                if path.extension() == Some(&OsStr::new("json5")){
                    let oss: &OsStr = path.file_stem().ok_or_else(|| format!("file stem couldn't be read {:?}", &de))?;
                    let file_stem_ref = oss.to_str().ok_or_else(|| format!("os_string couldn't be converted to a rust string {:?}", oss))?;
                    let file_stem = file_stem_ref.to_string();

                    let mut file = match File::open(de.path()) {
                        Ok(f) => f,
                        Err(_e) => { continue; }
                    };
                    let mut buf = String::new();
                    match file.read_to_string(&mut buf) {
                        Ok(_) => vec.push(JsonFileImpl { json: buf, filename_without_ext: file_stem }),
                        Err(e) => { Err(format!("{} couldn't be read", e))?; }
                    };
                }
            },
            Err(_e) =>{
                //???
                //Err(format!("{}", e.to_string()))?;
            }
        }
    }

    json_files_to_rust(vec.iter(), validation)
}

pub fn json_files_to_rust<T : JsonFile>(ite : impl Iterator<Item = T>, validation : bool) -> CoreResult<RootObject>{
    let mut map : HashM<String, RootValue> = HashMt::new();
    let mut root= None;

    for file in ite{
        let name = file.filename_without_ext();
        if name == "root"{
            if root.is_none() {
                root = Some(json_root_to_rust(file.json())?);
            } else{
                Err("There's two 'root.json5's in the directory")? //unreachableだけど一応
            }
        } else{
            match json_item_str_to_rust(file.json(), name){
                Ok(val) =>{ map.insert(name.to_string(), val.into_root_value2(name)?); }
                Err(e) =>{ Err(format!("filename {}, {}", name, e.to_string()))? }
            }
        }
    }

    if root.is_none(){
        Err("root.json5 is needed")?
    }

    return construct_root(root.unwrap(), map, validation);
}