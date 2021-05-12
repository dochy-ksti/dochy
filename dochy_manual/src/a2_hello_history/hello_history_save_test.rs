use dochy::error::DpResult;
use dochy::core::json_dir_to_root;
use crate::a2_hello_history::hello_history_accessor::RootIntf;
use dochy::fs::filesys::{save_file};
use dochy::fs::common::CurrentSrc;
use std::path::PathBuf;
use dochy::fs::history::{save_history_file, DochyCache, HistoryOptions, list_histories};

#[test]
fn hello_history_save_test() -> DpResult<()> {
    let root = json_dir_to_root("src/a2_hello_history/src_dir", true)?;

    let mut root = RootIntf::new(root);
    root.set_data2("data2'".to_string());

    let history_dir = "src/a2_hello_history/history_dir";
    let his = list_histories(history_dir)?;
    his.remove_old_files(0, history_dir);
    let mut cache = DochyCache::new(
        CurrentSrc::SrcDir(PathBuf::from(history_dir)),
        true, true);
    let opt = HistoryOptions::new();
    save_history_file(history_dir, None, root.root_obj_ref(), &mut cache, &opt);

    root.set_data3("data3'".to_string());
    save_history_file(history_dir, None, root.root_obj_ref(), &mut cache, &opt);
    for dir in list_histories(history_dir)?.list_dirs(){

    }
    std::fs::read_dir(files)
    // root.set_message("Hello the next world".to_string());
    //
    // let saved_path = save_file(
    //     "src/a1_hello_world/save_dir",
    //     root.root_obj_ref(),
    //     &CurrentSrc::SrcDir(PathBuf::from("src/a1_hello_world/some_dir")),
    //     "next_world.dochy",
    //     true)?;
    //
    // println!("{:?}", &saved_path);

    Ok(())
}