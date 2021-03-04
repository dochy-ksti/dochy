use nougami_intf::generate_interface;
use nougami_intf::util::nougami_intf_write_file::nougami_intf_write_file;
use nougami_core::structs::RootObject;
use crate::testing::diff::util::get_root_obj::get_root_obj;

pub fn generate_intf_src(json_dir_path : &str, src_file_path : &str) -> Result<RootObject, String> {
    let mut root = get_root_obj(json_dir_path)?;
    let source = generate_interface(&mut root);
    nougami_intf_write_file(src_file_path, &source.to_string_with_cfg_test());
    Ok(root)
}


