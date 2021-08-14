// use dochy::error::DpResult;
// use dochy::core::json_dir_to_root;
// use dochy::fs::common::{CurrentSrc, hash_dir_path, DochyCache};
// use std::path::{Path};
// use dochy::fs::history::{save_history_file, list_histories, load_history_file};
// use dochy::core::structs::RootObject;
// use crate::history_accum_test::hello_history_accessor::RootIntf;
//
//
// //#[test]
// fn accum_save_test() -> DpResult<()> {
//     let src_dir = "src/history_accum_test/src_dir";
//     let mut root = json_dir_to_root(src_dir, true)?;
//
//     let history_dir = Path::new("src/history_accum_test/history_dir");
//
//     std::fs::create_dir(history_dir).ok();
//     let his = list_histories(history_dir, ())?;
//     his.remove_old_files(0, history_dir)?;
//     let mut cache = DochyCache::new(
//         CurrentSrc::from_src_dir(src_dir))?;
//
//     for counter in 0..40{
//         save_twice(history_dir, root, counter, &mut cache)?;
//         root = load_prev_file(history_dir, &mut cache)?;
//     }
//     let his = list_histories(history_dir, ())?;
//     let d = his.get_newest_file_data()?;
//
//     let r = load_history_file(history_dir, d.hash(), d.props(), d.history(), &mut cache, (), false)?;
//     let r = RootIntf::new(r);
//     println!("{}", r.data1());
//
//     for file in &his.list_files(){
//         dbg!(file.calc_path("hoge"));
//     }
//
//     print_file_data(hash_dir_path(history_dir, d.hash()))?;
//     Ok(())
// }
//
// fn print_file_data<P : AsRef<Path>>(path : P) -> DpResult<()>{
//     for file in std::fs::read_dir(path)?{
//         let file = file?;
//         println!("{} size {}", file.file_name().to_str()?, file.metadata()?.len());
//     }
//     Ok(())
// }
//
// fn save_twice(history_dir : &Path, root : RootObject, counter : usize, cache : &mut DochyCache) -> DpResult<()>{
//     let mut r = RootIntf::new(root);
//     let mut txt = r.data1();
//     txt.push_str(&format!("{}", counter));
//     r.set_data1(txt.clone());
//     let _props = save_history_file(history_dir, None, r.root_obj_ref(), cache, ())?;
//     //dbg!(format!("1 {} {}", props.calc_filename(), counter));
//     txt.push_str(&format!("{}", counter));
//     r.set_data1(txt);
//     let _props = save_history_file(history_dir, None, r.root_obj_ref(), cache, ())?;
//     //dbg!(format!("2 {} {}", props.calc_filename(), counter));
//     Ok(())
// }
//
// fn load_prev_file(history_dir : &Path, cache : &mut DochyCache) -> DpResult<RootObject>{
//     let his = list_histories(history_dir, ())?;
//     let v = his.list_files();
//     let d = v.get(v.len() - 2)?;
//     //dbg!(d.calc_path("hoge"));
//     Ok(load_history_file(history_dir, d.hash(), d.props(), d.history(), cache, (), false)?)
// }