// use dochy::error::DpResult;
// use crate::a1_hello_world::hello_world_accessor::RootIntf;
// use dochy::fs::filesys::{ load_saved_file, list_saved_files};
// use dochy::fs::common::CurrentSrc;
// use std::path::PathBuf;
//
// #[test]
// fn hello_world_load_test() -> DpResult<()> {
//     let save_dir = "src/a1_hello_world/save_dir";
//     let files = list_saved_files(save_dir)?;
//
//     let file = files.iter().find(|f| f.name() == "next_world.dochy")?;
//
//     let root = load_saved_file(
//         file.calc_path(save_dir),
//         &CurrentSrc::SrcDir(PathBuf::from("src/a1_hello_world/some_dir")),
//         true
//     )?;
//     let root = RootIntf::new(root);
//     assert_eq!(root.message(), "Hello the next world");
//
//     Ok(())
// }