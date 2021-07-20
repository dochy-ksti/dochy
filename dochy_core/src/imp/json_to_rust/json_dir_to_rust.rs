
use std::fs::File;

use std::io::prelude::*;
use crate::error::CoreResult;
use std::ffi::{OsStr};
use crate::imp::json_to_rust::{json_root_to_rust, json_item_str_to_rust};
use crate::{HashM, HashMt};

use crate::imp::json_to_rust::construct_root::construct_root;
use crate::imp::structs::root_obj::RootObject;
use crate::imp::structs::json_file::{JsonFile, JsonFileImpl};
use crate::imp::structs::root_value::RootValue;
use std::path::Path;
use crate::imp::structs::var_type::VarType;
use crate::imp::structs::list_sab_value::ListSabValue;
use crate::imp::structs::root_sab_value::RootSabValue;
use crate::imp::json_to_rust::json_name::{json_name, NameType};

/// Converts Dochy source files to RootObject
/// Does extra checks when validation=true
pub fn json_dir_to_root<P : AsRef<Path>>(dir_path : P, validation : bool) -> CoreResult<RootObject>{
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

    json_files_to_root(vec.iter(), validation)
}

/// Converts Dochy source files to RootObject
/// Does extra checks when validation=true
pub fn json_files_to_root<T : JsonFile>(ite : impl Iterator<Item = T>, validation : bool) -> CoreResult<RootObject>{
    let mut vec : Vec<(String, RootValue, Option<RootSabValue>)> = Vec::new();
    //let mut sabun : HashM<String, RootSabValue> = HashMt::new();
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
            let name_type = json_name(name).ok_or_else(|| format!("filename:{} is not a valid name", name))?;
            match name_type {
                NameType::Name(name, var_type) => {
                    match json_item_str_to_rust(file.json(), &name, var_type.clone()) {
                        Ok(val) => {
                            let (v, sab) = val.into_root_value2(&name)?;
                            vec.push((name, v, sab));
                        },
                        Err(e) => { Err(format!("filename {}, {}", name, e.to_string()))? }
                    }
                },
                NameType::SystemName(s) =>{
                    Err(format!("filename {} can't be used", name))?
                }
            }
        }
    }

    if root.is_none(){
        Err("root.json5 is needed")?
    }

    return construct_root(root.unwrap(), vec, validation);
}