use dochy::error::DpResult;
use crate::a1_hello_world::hello_world_accessor::RootIntf;
use dochy::fs::filesys::{save_dochy_file, SaveDirInfo, list_dochy_files, load_dochy_file};
use dochy::fs::common::{CurrentSrc};

#[test]
fn save_dochy_files_test() -> DpResult<()> {
    // Let's save and load Dochy File

    // Dochy has two file formats, Dochy File and Dochy History File.
    // Dochy File is the simplest format of the two.

    let save_dir = "src/b1_save_dochy_files/save_dir";

    // make sure the save_dir exists.
    std::fs::create_dir(save_dir).ok();

    let src_dir = "src/b1_save_dochy_files/src_dir";

    // You need SaveDirInfo to save Dochy-files.
    // save_dir and src_dir's paths are needed to create SaveDirInfo.
    let info = SaveDirInfo::create(save_dir, CurrentSrc::from_src_dir(src_dir))?;
    //SaveDirInfo::create returns FsResult, which is the result type of the module dochy_fs.
    //It's automatically converted to DpResult with ? operator.
    //Every other result type is automatically converted to DpResult, so basically, users of this library only need DpResult.


    // RootObject is created from Dochy-source. It can be cloned via SaveDirInfo
    let root = info.clone_src_root();
    let mut root = RootIntf::new(root);
    root.set_message("Hello the next world".to_string());

    let _saved_path = save_dochy_file(
        &info,
        "next_world.dochy",
        root.root_obj_ref(),
        true)?;

    hello_world_load_test()?;

    Ok(())
}

fn hello_world_load_test() -> DpResult<()> {
    let save_dir = "src/a1_hello_world/save_dir";
    let src_dir = "src/a1_hello_world/src_dir";
    let info = SaveDirInfo::create(save_dir, CurrentSrc::from_src_dir(src_dir))?;


    let files = list_dochy_files(save_dir)?;

    let file = files.iter().find(|f| f.name() == "next_world.dochy")?;

    let root = load_dochy_file(
        file.calc_path(save_dir),
        &info,
        true
    )?;
    let root = RootIntf::new(root);
    assert_eq!(root.message(), "Hello the next world");

    Ok(())
}