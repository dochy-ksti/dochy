use dochy::error::DpResult;
use crate::a1_hello_world::hello_world_accessor::RootIntf;
use dochy::fs::filesys::{save_dochy_file};
use dochy::fs::common::{CurrentSrc};

#[test]
fn hello_world_save_test() -> DpResult<()> {
    let current_src = CurrentSrc::from_src_dir("src/a1_hello_world/some_dir");
    let (src_root, hash) = current_src.create_root_and_hash(false)?;
    let root = src_root.clone();

    let mut root = RootIntf::new(root);
    root.set_message("Hello the next world".to_string());

    let save_dir_path = "src/a1_hello_world/save_dir";
    std::fs::create_dir(save_dir_path).ok();

    let _saved_path = save_dochy_file(
        save_dir_path,
        "next_world.dochy",
        root.root_obj_ref(),
        &current_src,
        hash,
        &src_root,
        true)?;

    //println!("{:?}", &saved_path);

    Ok(())
}