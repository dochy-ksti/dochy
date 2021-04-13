use dochy::core::json_dir_to_root;
use dochy::core::structs::RootObject;

pub fn get_root_obj(json_dir_path: &str) -> Result<RootObject, String> {
    json_dir_to_root(json_dir_path, true).or_else(|e| Err(e.to_string()))
}