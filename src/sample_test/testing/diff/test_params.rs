//
//
// #[cfg(test)]
// mod tests {
//     use crate::testing::diff::util::get_root_obj::get_root_obj;
//     use nougami_core::intf::null_or::{NullOr, UndefOr};
//     use nougami_core::structs::Qv;
//     use crate::testing::diff::generated_test_params::test::RootIntf;
//
//     #[test]
//     fn test_params(){
//         match test_diff2(){
//             Ok(()) =>{},
//             Err(s) => println!("{}", s),
//         }
//
//         match test_diff3(){
//             Ok(()) =>{},
//             Err(s) => println!("{}", s),
//         }
//     }
//
//     fn test_diff2() -> Result<(), String>{
//         let json_dir_path = "src/testing/diff/diff_param/";
//         let root_obj = get_root_obj(json_dir_path)?;
//
//         let mut intf = RootIntf::new(root_obj);
//
//         intf.set_hoge_int(1);
//         intf.set_hoge_float(1.0);
//         intf.set_hoge_string("山原や".to_string());
//         intf.set_hoge_bool(false);
//         intf.set_hoge_int_array(vec![]);
//         intf.set_hoge_float_array(vec![1.2, 3.4, 5.6, 7.8]);
//         intf.set_hoge_int_hatena(NullOr::Val(1));
//         intf.set_hoge_int_hatena_to_null(NullOr::Null);
//         intf.set_hoge_int_hatena_null(NullOr::Val(1));
//         intf.set_hoge_undefined(UndefOr::Val(1));
//         intf.set_hoge_undefined_to_undef(UndefOr::Undefined);
//         intf.set_hoge_undefined_undef(UndefOr::Val(1));
//         intf.set_hoge_undef_null_to_null(Qv::Null);
//         intf.set_hoge_undef_null_to_undef(Qv::Undefined);
//         intf.set_hoge_undef_null_null(Qv::Val(1));
//         intf.set_hoge_undef_null_undef(Qv::Val(1));
//         intf.set_hoge_undef_null_null_to_undef(Qv::Undefined);
//         intf.set_hoge_undef_null_undef_to_null(Qv::Null);
//
//         let moto = get_root_obj(json_dir_path)?;
//
//         let diff = nougami_diff::get_diff(&moto, unsafe{ intf.root_obj_ref() }).or_else(|e| Err(e.message))?;
//         let applied = nougami_diff::apply_diff(moto, diff).or_else(|e| Err(e.message))?;
//         let intf = RootIntf::new(applied);
//
//
//         assert_eq!(intf.hoge_int(), 1);
//         assert_eq!(intf.hoge_float(), 1.0);
//         assert_eq!(intf.hoge_string(), "山原や");
//         assert_eq!(intf.hoge_bool(), false);
//         assert_eq!(intf.hoge_int_array(), vec![]);
//         assert_eq!(intf.hoge_float_array(), vec![1.2,3.4,5.6,7.8]);
//         assert_eq!(intf.hoge_int_hatena(), NullOr::Val(1));
//         assert_eq!(intf.hoge_int_hatena_to_null(), NullOr::Null);
//         assert_eq!(intf.hoge_int_hatena_null(), NullOr::Val(1));
//         assert_eq!(intf.hoge_undefined(), UndefOr::Val(1));
//         assert_eq!(intf.hoge_undefined_to_undef(), UndefOr::Undefined);
//         assert_eq!(intf.hoge_undefined_undef(), UndefOr::Val(1));
//         assert_eq!(intf.hoge_undef_null_to_null(), Qv::Null);
//         assert_eq!(intf.hoge_undef_null_to_undef(), Qv::Undefined);
//         assert_eq!(intf.hoge_undef_null_null(), Qv::Val(1));
//         assert_eq!(intf.hoge_undef_null_undef(), Qv::Val(1));
//         assert_eq!(intf.hoge_undef_null_null_to_undef(), Qv::Undefined);
//         assert_eq!(intf.hoge_undef_null_undef_to_null(), Qv::Null);
//         assert_eq!(intf.hoge_int(), 1);
//         assert_eq!(intf.hoge_int(), 1);
//         Ok(())
//     }
//
//     //二段階diffする
//     fn test_diff3() -> Result<(), String>{
//         let json_dir_path = "src/testing/diff/diff_param/";
//         let root_obj = get_root_obj(json_dir_path)?;
//
//         let mut intf = RootIntf::new(root_obj);
//
//         intf.set_hoge_int(2);
//         intf.set_hoge_float(2.0);
//         intf.set_hoge_string("山原やん".to_string());
//         intf.set_hoge_bool(true);
//         intf.set_hoge_int_array(vec![100]);
//         intf.set_hoge_float_array(vec![1.12, 3.14, 5.16, 7.18]);
//         intf.set_hoge_int_hatena(NullOr::Val(2));
//         intf.set_hoge_int_hatena_to_null(NullOr::Val(3));
//         intf.set_hoge_int_hatena_null(NullOr::Val(2));
//         intf.set_hoge_undefined(UndefOr::Val(2));
//         intf.set_hoge_undefined_to_undef(UndefOr::Val(4));
//         intf.set_hoge_undefined_undef(UndefOr::Val(2));
//         intf.set_hoge_undef_null_to_null(Qv::Val(3));
//         intf.set_hoge_undef_null_to_undef(Qv::Val(4));
//         intf.set_hoge_undef_null_null(Qv::Val(2));
//         intf.set_hoge_undef_null_undef(Qv::Val(2));
//         intf.set_hoge_undef_null_null_to_undef(Qv::Val(4));
//         intf.set_hoge_undef_null_undef_to_null(Qv::Val(3));
//
//         let moto = get_root_obj(json_dir_path)?;
//
//         let diff1 = nougami_diff::get_diff(&moto, unsafe{ intf.root_obj_ref() }).or_else(|e| Err(e.message))?;
//         let applied1 = nougami_diff::apply_diff(moto, diff1).or_else(|e| Err(e.message))?;
//
//         intf.set_hoge_int(1);
//         intf.set_hoge_float(1.0);
//         intf.set_hoge_string("山原や".to_string());
//         intf.set_hoge_bool(false);
//         intf.set_hoge_int_array(vec![]);
//         intf.set_hoge_float_array(vec![1.2, 3.4, 5.6, 7.8]);
//         intf.set_hoge_int_hatena(NullOr::Val(1));
//         intf.set_hoge_int_hatena_to_null(NullOr::Null);
//         intf.set_hoge_int_hatena_null(NullOr::Val(1));
//         intf.set_hoge_undefined(UndefOr::Val(1));
//         intf.set_hoge_undefined_to_undef(UndefOr::Undefined);
//         intf.set_hoge_undefined_undef(UndefOr::Val(1));
//         intf.set_hoge_undef_null_to_null(Qv::Null);
//         intf.set_hoge_undef_null_to_undef(Qv::Undefined);
//         intf.set_hoge_undef_null_null(Qv::Val(1));
//         intf.set_hoge_undef_null_undef(Qv::Val(1));
//         intf.set_hoge_undef_null_null_to_undef(Qv::Undefined);
//         intf.set_hoge_undef_null_undef_to_null(Qv::Null);
//
//         let diff2 = nougami_diff::get_diff(&applied1, unsafe{ intf.root_obj_ref() }).or_else(|e| Err(e.message))?;
//         let applied2 = nougami_diff::apply_diff(applied1, diff2).or_else(|e| Err(e.message))?;
//
//         let intf = RootIntf::new(applied2);
//
//         assert_eq!(intf.hoge_int(), 1);
//         assert_eq!(intf.hoge_float(), 1.0);
//         assert_eq!(intf.hoge_string(), "山原や");
//         assert_eq!(intf.hoge_bool(), false);
//         assert_eq!(intf.hoge_int_array(), vec![]);
//         assert_eq!(intf.hoge_float_array(), vec![1.2,3.4,5.6,7.8]);
//         assert_eq!(intf.hoge_int_hatena(), NullOr::Val(1));
//         assert_eq!(intf.hoge_int_hatena_to_null(), NullOr::Null);
//         assert_eq!(intf.hoge_int_hatena_null(), NullOr::Val(1));
//         assert_eq!(intf.hoge_undefined(), UndefOr::Val(1));
//         assert_eq!(intf.hoge_undefined_to_undef(), UndefOr::Undefined);
//         assert_eq!(intf.hoge_undefined_undef(), UndefOr::Val(1));
//         assert_eq!(intf.hoge_undef_null_to_null(), Qv::Null);
//         assert_eq!(intf.hoge_undef_null_to_undef(), Qv::Undefined);
//         assert_eq!(intf.hoge_undef_null_null(), Qv::Val(1));
//         assert_eq!(intf.hoge_undef_null_undef(), Qv::Val(1));
//         assert_eq!(intf.hoge_undef_null_null_to_undef(), Qv::Undefined);
//         assert_eq!(intf.hoge_undef_null_undef_to_null(), Qv::Null);
//         assert_eq!(intf.hoge_int(), 1);
//         assert_eq!(intf.hoge_int(), 1);
//         Ok(())
//     }
//
//
// }