use dochy::error::DpResult;
use crate::a1_hello_world::hello_world_accessor::RootIntf;
use dochy::fs::filesys::{save_file};
use dochy::fs::common::{CurrentSrc, DochyCache};
use std::path::PathBuf;

#[test]
fn hello_world_save_test() -> DpResult<()> {
    let cache = DochyCache::new(CurrentSrc::SrcDir(PathBuf::from("src/a1_hello_world/some_dir")))?;
    let root = cache.clone_src_root();

    let mut root = RootIntf::new(root);
    root.set_message("Hello the next world".to_string());

    let save_dir_path = "src/a1_hello_world/save_dir";
    std::fs::create_dir(save_dir_path).ok();

    let _saved_path = save_file(
        save_dir_path,
        root.root_obj_ref(),
        &cache,
        "next_world.dochy",
        true)?;

    //println!("{:?}", &saved_path);

    Ok(())
}