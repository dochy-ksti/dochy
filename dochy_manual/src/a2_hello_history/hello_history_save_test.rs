use dochy::error::DpResult;
use dochy::core::json_dir_to_root;
use crate::a2_hello_history::hello_history_accessor::RootIntf;
use dochy::fs::filesys::{save_file};
use dochy::fs::common::CurrentSrc;
use std::path::{PathBuf, Path};
use dochy::fs::history::{save_history_file, DochyCache, list_histories, load_history_file};

#[test]
fn hello_history_save_test() -> DpResult<()> {
    let src_dir = "src/a2_hello_history/src_dir";
    let root = json_dir_to_root(src_dir, true)?;


    let history_dir = "src/a2_hello_history/history_dir";

    std::fs::create_dir(history_dir).ok();
    let his = list_histories(history_dir, ())?;
    his.remove_old_files(0, history_dir)?;
    let mut cache = DochyCache::new(
        CurrentSrc::from_src_dir(src_dir));

    let mut root = RootIntf::new(root);
    root.set_data2("data2'".to_string());

    save_history_file(history_dir, None, root.root_obj_ref(), &mut cache, ())?;

    let his = list_histories(history_dir, ())?;
    let file_data = his.get_newest_file_data().unwrap();

    let loaded1 = load_history_file(history_dir, file_data.hash(), file_data.props(), file_data.history(), &mut cache, (), false)?;
    let mut loaded1 = RootIntf::new(loaded1);
    assert_eq!(loaded1.data2(), "data2'".to_string());

    root.set_data3("data3'".to_string());
    save_history_file(history_dir, None, root.root_obj_ref(), &mut cache, ())?;

    let loaded2 = load_history_file(history_dir, file_data.hash(), file_data.props(), file_data.history(), &mut cache, (), false)?;
    let mut loaded2 = RootIntf::new(loaded2);
    assert_eq!(loaded2.data2(), "data2'".to_string());
    assert_eq!(loaded2.data3(), "data3'".to_string());



    Ok(())
}

fn print_file_data<P : AsRef<Path>>(path : P) -> DpResult<()>{
    for file in std::fs::read_dir(path)?{
        let file = file?;
        println!("{} size {}", file.file_name().to_str()?, file.metadata()?.len());
    }
    Ok(())
}