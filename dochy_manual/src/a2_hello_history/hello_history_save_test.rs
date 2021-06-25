use dochy::error::DpResult;
use dochy::core::json_dir_to_root;
use crate::a2_hello_history::hello_history_accessor::RootIntf;
use dochy::fs::filesys::{save_file};
use dochy::fs::common::CurrentSrc;
use std::path::{PathBuf, Path};
use dochy::fs::history::{save_history_file, DochyCache, list_histories};

#[test]
fn hello_history_save_test() -> DpResult<()> {
    let src_dir = "src/a2_hello_history/src_dir";
    let root = json_dir_to_root(src_dir, true)?;

    let mut root = RootIntf::new(root);
    root.set_data2("data2'".to_string());

    let history_dir = "src/a2_hello_history/history_dir";

    std::fs::create_dir(history_dir).ok();
    let his = list_histories(history_dir, ())?;
    his.remove_old_files(0, history_dir)?;
    let mut cache = DochyCache::new(
        CurrentSrc::from_src_dir(src_dir));
    save_history_file(history_dir, None, root.root_obj_ref(), &mut cache)?;

    root.set_data3("data3'".to_string());
    save_history_file(history_dir, None, root.root_obj_ref(), &mut cache)?;

    let paths : Vec<PathBuf> = dochy::fs::common::hash_dir_paths(history_dir)?.collect();
    assert_eq!(paths.len(), 1);

    //print_file_data(&paths[0])?;

    let root = json_dir_to_root(src_dir, true)?;

    let mut root = RootIntf::new(root);
    root.set_data2("data2'".to_string());

    let save_dir = "src/a2_hello_history/save_dir";
    std::fs::create_dir(save_dir).ok();

    save_file(
        save_dir,
        root.root_obj_ref(),
        &CurrentSrc::from_src_dir(src_dir),
        "d1.dochy",
        true)?;

    root.set_data3("data3'".to_string());

    save_file(
        save_dir,
        root.root_obj_ref(),
        &CurrentSrc::from_src_dir(src_dir),
        "d2.dochy",
        true)?;

    let paths : Vec<PathBuf> = dochy::fs::common::hash_dir_paths(save_dir)?.collect();
    assert_eq!(paths.len(), 1);

    //print_file_data(&paths[0])?;
    Ok(())
}

fn print_file_data<P : AsRef<Path>>(path : P) -> DpResult<()>{
    for file in std::fs::read_dir(path)?{
        let file = file?;
        println!("{} size {}", file.file_name().to_str()?, file.metadata()?.len());
    }
    Ok(())
}