use dochy::error::DpResult;
use dochy::fs::history::{HistoryInfo, save_history_file, list_histories};
use dochy::fs::common::CurrentSrc;
use crate::save_1gb_100::save_1gb_accessor::RootIntf;
use rand::rngs::ThreadRng;
use rand::Rng;
use std::time::Instant;
use std::path::Path;
use crate::util::get_file_lens;
use serde::{Serialize, Deserialize};
use crate::save_1gb_100::save_1gb_bench::VEC_SIZE;

const MB: usize = 10_000;
const LOOP : usize = 10;

#[derive(Serialize, Deserialize)]
struct DataJson {
    data0: String,
    data1: String,
    data2: String,
    data3: String,
    data4: String,
    data5: String,
    data6: String,
    data7: String,
    data8: String,
    data9: String,
}

#[test]
fn save_1gb_json_bench() -> DpResult<()> {
    let json_dir = "src/save_1gb_100/json_dir";
    std::fs::remove_dir_all(json_dir).ok();
    std::fs::create_dir(json_dir).ok();


    init();
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

fn modify(d : &mut DataJson){
    let mut rng = rand::thread_rng();
    let r : usize = rng.gen_range(0..10);
    match r{
        0 => d.data0 = random_str(),
        1 => d.data1 = random_str(),
        2 => d.data2 = random_str(),
        3 => d.data3 = random_str(),
        4 => d.data4 = random_str(),
        5 => d.data5 = random_str(),
        6 => d.data6 = random_str(),
        7 => d.data7 = random_str(),
        8 => d.data8 = random_str(),
        9 => d.data9 = random_str(),

        _ => unreachable!(),
    }
}

fn init() -> DataJson{
    DataJson{
        data0 : random_str(),
        data1 : random_str(),
        data2 : random_str(),
        data3 : random_str(),
        data4 : random_str(),
        data5 : random_str(),
        data6 : random_str(),
        data7 : random_str(),
        data8 : random_str(),
        data9 : random_str(),
    };

}

fn random_str() -> String{
    let mut s = String::with_capacity(VEC_SIZE);
    let mut rng = rand::thread_rng();
    for _ in 0..VEC_SIZE {
        s.push(rng.gen_range('!'..='~'));
    }
    v
}