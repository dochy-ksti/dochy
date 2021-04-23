// use dochy::core::json_dir_to_root;
// use crate::sample_test::sample_code::ref1_accessor::RootIntf;
// use dochy::error::DpResult;
//
// #[test]
// fn ref1_test() -> DpResult<()> {
//     let old = json_dir_to_root("src/sample_test/sample_code_json/ref1", true)?;
//
//     let mut r = RootIntf::new(old);
//     let mut list = r.list();
//     //mlist is linked-hash-map, which is hashtable whose items are doubly-linked-list-node.
//     //so first() and last() can be done instantly, but getting middle items are slow,
//     //unless you use find_by_id instead.
//     //Linked-hash-map is also hashmap, so you can find items by key(ID) super fast.
//     let item = list.last()?;
//     assert_eq!(item.ref_table_a().foo(), 33);
//     Ok(())
// }