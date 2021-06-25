use dochy::error::DpResult;
use dochy::core::json_dir_to_root;
use crate::a1_hello_world::hello_world_accessor::RootIntf;
use dochy::fs::filesys::{save_file};
use dochy::fs::common::CurrentSrc;
use std::path::PathBuf;

#[test]
fn hello_world_save_test() -> DpResult<()> {
    let root = json_dir_to_root("src/a1_hello_world/some_dir", true)?;

    let mut root = RootIntf::new(root);
    root.set_message("Hello the next world".to_string());

    std::fs::create_dir("src/a1_hello_world/save_dir").ok();

    let _saved_path = save_file(
        "src/a1_hello_world/save_dir",
        root.root_obj_ref(),
        &CurrentSrc::SrcDir(PathBuf::from("src/a1_hello_world/some_dir")),
        "next_world.dochy",
        true)?;

    //println!("{:?}", &saved_path);

    Ok(())
}