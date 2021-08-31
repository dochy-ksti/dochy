use dochy::error::DpResult;
use dochy::fs::history::{HistoryInfo, save_history_file, list_histories};
use dochy::fs::common::CurrentSrc;
use crate::save_1gb_100::save_1gb_accessor::RootIntf;
use rand::rngs::ThreadRng;
use rand::Rng;
use std::time::Instant;
use std::path::Path;
use crate::util::get_file_lens;

pub const VEC_SIZE: usize = 10_000;
pub const LOOP : usize = 10;

#[test]
fn save_1gb_bench() -> DpResult<()> {
    let history_dir = "src/save_1gb_100/history_dir";
    std::fs::remove_dir_all(history_dir).ok();
    std::fs::create_dir(history_dir).ok();

    let src_dir = "src/save_1gb_100/src_dir";
    let info = HistoryInfo::create(history_dir,
                                   CurrentSrc::from_src_dir(src_dir), ())?;

    let root = info.clone_src_root();
    let mut root = RootIntf::new(root);
    init(&mut root);
    let start_time = Instant::now();
    for _ in 0..LOOP{
        modify(&mut root);
        save_history_file(&info, None, root.root_obj_ref())?;
    }
    let end_time = Instant::now();
    let duration = end_time - start_time;
    println!("{} milliseconds", duration.as_millis());

    let his = list_histories(&info)?;
    let hash_dir = his.get_newest_file_data()?.calc_path(history_dir).parent()?.to_path_buf();
    let lens = get_file_lens(&hash_dir)?;
    let sum_files = lens.iter().fold(0, |sum, fl| sum + fl.len());
    println!("sum of file size {}", sum_files);
    for fl in &lens{
        println!("file {} len {}", fl.file_name(), fl.len())
    }
    Ok(())
}

fn modify(root : &mut RootIntf){
    let mut rng = rand::thread_rng();
    let r : usize = rng.gen_range(0..10);
    match r{
        0 => root.set_data0(initial_vec()),
        1 => root.set_data1(initial_vec()),
        2 => root.set_data2(initial_vec()),
        3 => root.set_data3(initial_vec()),
        4 => root.set_data4(initial_vec()),
        5 => root.set_data5(initial_vec()),
        6 => root.set_data6(initial_vec()),
        7 => root.set_data7(initial_vec()),
        8 => root.set_data8(initial_vec()),
        9 => root.set_data9(initial_vec()),
        _ => unreachable!(),
    }
}

fn init(root : &mut RootIntf){
    root.set_data0(initial_vec());
    root.set_data1(initial_vec());
    root.set_data2(initial_vec());
    root.set_data3(initial_vec());
    root.set_data4(initial_vec());
    root.set_data5(initial_vec());
    root.set_data6(initial_vec());
    root.set_data7(initial_vec());
    root.set_data8(initial_vec());
    root.set_data9(initial_vec());
}

fn initial_vec() -> Vec<u8>{
    let mut v = Vec::with_capacity(VEC_SIZE);
    let mut rng = rand::thread_rng();
    for _ in 0..VEC_SIZE {
        v.push(rng.gen_range('!'..='~' as u8));
    }
    v
}