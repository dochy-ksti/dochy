use dochy::error::DpResult;
use crate::a1_hello_world::hello_world_accessor::RootIntf;
use dochy::fs::filesys::{ load_saved_file, list_saved_files};
use dochy::fs::common::{CurrentSrc};

#[test]
fn hello_world_load_test() -> DpResult<()> {
    let save_dir = "src/a1_hello_world/save_dir";
    let current_src = CurrentSrc::from_src_dir("src/a1_hello_world/some_dir");
    let (src_root, hash) = current_src.create_root_and_hash(false)?;
    let files = list_saved_files(save_dir)?;

    let file = files.iter().find(|f| f.name() == "next_world.dochy")?;

    let root = load_saved_file(
        file.calc_path(save_dir),
        hash,
        &src_root,
        true
    )?;
    let root = RootIntf::new(root);
    assert_eq!(root.message(), "Hello the next world");

    Ok(())
}