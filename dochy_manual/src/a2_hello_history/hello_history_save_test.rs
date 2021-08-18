use dochy::error::DpResult;
use dochy::core::json_dir_to_root;
use crate::a2_hello_history::hello_history_accessor::RootIntf;
use dochy::fs::common::{CurrentSrc, hash_dir_path, DochyCache};
use std::path::{Path};
use dochy::fs::history::{save_history_file, list_histories, load_history_file, HistoryInfo};
use dochy::core::structs::RootObject;

///save_twiceしてsecond newest fileをload
//#[test]
fn hello_history_save_test() -> DpResult<()> {


    let info = HistoryInfo::create("src/a2_hello_history/history_dir",
                        CurrentSrc::from_src_dir("src/a2_hello_history/src_dir"), ())?;
    let history_dir = info.history_dir();
    std::fs::create_dir(history_dir).ok();
    let his = list_histories(history_dir, ())?;
    his.remove_old_files(0, history_dir)?;


    for counter in 0..40{
        save_twice(&info, root, counter)?;
        root = load_prev_file(&info)?;
    }
    let his = list_histories(history_dir, ())?;
    let d = his.get_newest_file_data()?;

    let r = load_history_file(&info d.props(), d.history(), false)?;
    let r = RootIntf::new(r);
    println!("{}", r.data1());

    for file in &his.list_files(){
        dbg!(file.calc_path("hoge"));
    }

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

fn save_twice(info : &HistoryInfo, root : RootObject, counter : usize) -> DpResult<()>{
    let mut r = RootIntf::new(root);
    let mut txt = r.data1().clone();
    txt.push_str(&format!("{}", counter));
    r.set_data1(txt.clone());
    let _props = save_history_file(info, None, r.root_obj_ref())?;
    //dbg!(format!("1 {} {}", props.calc_filename(), counter));
    txt.push_str(&format!("{}", counter));
    r.set_data1(txt.clone());
    let _props = save_history_file(info, None, r.root_obj_ref())?;
    //dbg!(format!("2 {} {}", props.calc_filename(), counter));
    Ok(())
}

fn load_prev_file(info : &HistoryInfo) -> DpResult<RootObject>{
    let history_dir = info.history_dir();
    let his = list_histories(history_dir, ())?;
    let v = his.list_files();
    let d = v.get(v.len() - 2)?;
    //dbg!(d.calc_path("hoge"));
    Ok(load_history_file(info, d.props(), d.history(), false)?)
}