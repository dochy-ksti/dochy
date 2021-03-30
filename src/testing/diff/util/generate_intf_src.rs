use dochy_intf::generate_interface;
use crate::testing::diff::util::get_root_obj::get_root_obj;
use dochy_core::structs::RootObject;
use crate::error::DpResult;

pub fn generate_intf_src(json_dir_path : &str, src_file_path : &str) -> DpResult<RootObject> {
    let mut root = get_root_obj(json_dir_path)?;
    let source = generate_interface(&mut root);
    std::fs::write(src_file_path, &source.to_string_with_cfg_test())?;

    Ok(root)
}


