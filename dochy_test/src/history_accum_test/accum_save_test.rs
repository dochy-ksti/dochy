use dochy::error::DpResult;
use dochy::core::json_dir_to_root;
use dochy::fs::common::{CurrentSrc, hash_dir_path, DochyCache};
use std::path::{Path};
use dochy::fs::history::{save_history_file, list_histories, load_history_file, CurrentRootObjInfo};
use dochy::core::structs::RootObject;
use crate::history_accum_test::hello_history_accessor::RootIntf;
use rand::Rng;

///10メンバのうち1メンバを上書きするので90%はそのままだが、
/// 各メンバ(Binary)がジワジワ伸びていくので全体のファイルサイズは大きくなっていく
/// そんな状態が正しくHistoryとして記録できているか、目視とload and compareで確認
#[test]
fn accum_save_test() -> DpResult<()> {
    let src_dir = "src/history_accum_test/src_dir";
    let root = json_dir_to_root(src_dir, true)?;

    let history_dir = Path::new("src/history_accum_test/history_dir");

    std::fs::create_dir(history_dir).ok();
    let his = list_histories(history_dir, ())?;
    his.remove_old_files(0, history_dir)?;
    let mut cache = DochyCache::new(
        CurrentSrc::from_src_dir(src_dir))?;
    let mut root = RootIntf::new(root);

    for _counter in 0..60{
        mutate_root(&mut root);
        let saved = save_history_file(history_dir, None, root.root_obj_ref(), &mut cache, ())?;
        let loaded = RootIntf::new(load_newest_file(history_dir, &mut cache)?);
        //mutate_root(&mut loaded);
        assert!(root.root_obj_ref().contents_eq(loaded.root_obj_ref()));
        //ロードするとロードしたオブジェクトでセーブしないとHistoryが構成できない
        //ここはインチキして、セーブした時の状態に戻す
        dochy::fs::history::set_current_root_obj_info(
            history_dir, cache.hash(), Some(CurrentRootObjInfo::new(root.root_obj_ref().id(), saved, true)))
    }

    print_file_data(hash_dir_path(history_dir, cache.hash()))?;
    Ok(())
}

fn mutate_root(r : &mut RootIntf){
    let m = match rand::thread_rng().gen_range(0..=9){
        0 => r.data0_mut(),
        1 => r.data1_mut(),
        2 => r.data2_mut(),
        3 => r.data3_mut(),
        4 => r.data4_mut(),
        5 => r.data5_mut(),
        6 => r.data6_mut(),
        7 => r.data7_mut(),
        8 => r.data8_mut(),
        9 => r.data9_mut(),
        _ => unreachable!(),
    };

    //IntArrayの場合は、乱数0を引いて変化なしの場合かなり小さなファイルサイズになる。
    //Binaryの場合、変化なしかどうかを比較して確かめないので、どうしても大きくなる
    let l = rand::thread_rng().gen_range(0..=5);
    for _ in 0..l{
        let k = rand::thread_rng().gen_range(0..=100);
        m.push(k);
    }
}

fn print_file_data<P : AsRef<Path>>(path : P) -> DpResult<()>{
    for file in std::fs::read_dir(path)?{
        let file = file?;
        println!("{} size {}", file.file_name().to_str()?, file.metadata()?.len());
    }
    Ok(())
}


fn load_newest_file(history_dir : &Path, cache : &mut DochyCache) -> DpResult<RootObject>{
    let his = list_histories(history_dir, ())?;
    let d = his.get_newest_file_data()?;

    Ok(load_history_file(history_dir, d.hash(), d.props(), d.history(), cache, (), false)?)
}