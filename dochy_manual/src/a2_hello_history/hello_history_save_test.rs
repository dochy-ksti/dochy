use dochy::error::DpResult;
use dochy::core::json_dir_to_root;
use crate::a2_hello_history::hello_history_accessor::RootIntf;
use dochy::fs::filesys::{save_file};
use dochy::fs::common::{CurrentSrc, hash_dir_path};
use std::path::{PathBuf, Path};
use dochy::fs::history::{save_history_file, DochyCache, list_histories, load_history_file};
use dochy::core::structs::RootObject;

#[test]
fn hello_history_save_test() -> DpResult<()> {
    let src_dir = "src/a2_hello_history/src_dir";
    let mut root = json_dir_to_root(src_dir, true)?;


    let history_dir = Path::new("src/a2_hello_history/history_dir");

    std::fs::create_dir(history_dir).ok();
    let his = list_histories(history_dir, ())?;
    his.remove_old_files(0, history_dir)?;
    let mut cache = DochyCache::new(
        CurrentSrc::from_src_dir(src_dir));

    for counter in 0..10{
        save_twice(history_dir, root, counter, &mut cache);
        root = load_prev_file(history_dir, &mut cache)?;
    }
    let his = list_histories(history_dir, ())?;
    let d = his.get_newest_file_data()?;

    print_file_data(hash_dir_path(history_dir, d.hash()))?;
    Ok(())
}

fn print_file_data<P : AsRef<Path>>(path : P) -> DpResult<()>{
    for file in std::fs::read_dir(path)?{
        let file = file?;
        println!("{} size {}", file.file_name().to_str()?, file.metadata()?.len());
    }
    Ok(())
}

fn save_twice(history_dir : &Path, root : RootObject, counter : usize, cache : &mut DochyCache) -> DpResult<()>{
    let mut r = RootIntf::new(root);
    let mut txt = r.data1();
    txt.push_str(&format!("{}", counter));
    r.set_data1(txt.clone());
    save_history_file(history_dir, None, r.root_obj_ref(), cache, ())?;
    txt.push_str(&format!("{}", counter+1));
    r.set_data1(txt);
    save_history_file(history_dir, None, r.root_obj_ref(), cache, ())?;
    Ok(())
}

fn load_prev_file(history_dir : &Path, cache : &mut DochyCache) -> DpResult<RootObject>{
    let his = list_histories(history_dir, ())?;
    let v = his.list_files();
    let d = v.get(v.len() - 2)?;
    Ok(load_history_file(history_dir, d.hash(), d.props(), d.history(), cache, (), false)?)
}