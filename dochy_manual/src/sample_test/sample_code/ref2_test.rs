// use dochy::core::json_dir_to_root;
// use crate::sample_test::sample_code::ref2_accessor::RootIntf;
// use dochy::error::DpResult;
//
// #[test]
// fn ref2_test() -> DpResult<()> {
//     let old = json_dir_to_root("src/sample_test/sample_code_json/ref2", true)?;
//
//     let mut r = RootIntf::new(old);
//     let mut list = r.list();
//
//     let item = list.last()?;
//     assert_eq!(item.ref_table_a().foo(), 33);
//     Ok(())
// }