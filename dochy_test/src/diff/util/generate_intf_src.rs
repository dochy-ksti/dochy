use dochy::intf::generate_interface;
use dochy::core::structs::RootObject;
use dochy::error::DpResult;
use dochy::core::json_dir_to_root;

pub fn generate_intf_src(json_dir_path : &str, src_file_path : &str) -> DpResult<RootObject> {
    let mut root = json_dir_to_root(json_dir_path,true)?;
    let source = generate_interface(&mut root);
    std::fs::write(src_file_path, &source.to_string_with_cfg_test())?;

    Ok(root)
}


