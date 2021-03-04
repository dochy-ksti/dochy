use nougami_core::json_dir_to_rust;
use nougami_core::structs::RootObject;

pub fn get_root_obj(json_dir_path: &str) -> Result<RootObject, String> {
    json_dir_to_rust(json_dir_path, true).or_else(|e| Err(e.message))
}