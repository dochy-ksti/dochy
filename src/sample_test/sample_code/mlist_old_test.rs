use dochy_core::json_dir_to_rust;
use crate::sample_test::sample_code::mlist_old_accessor::RootIntf;
use crate::error::DpResult;

#[test]
fn mlilst_old_test() -> DpResult<()> {
    let old = json_dir_to_rust("src/sample_test/sample_code_json/mlist_old", true)?;

    let mut r = RootIntf::new(old);
    let mut iter = r.mlist().iter();

    if let Some((_id, item)) = iter.next(){
        assert_eq!(item.bar(), 1);
        assert_eq!(item.baz(), 2);
    }
    if let Some((_id, item)) = iter.next(){
        assert_eq!(item.bar(), 0); // default value
        assert_eq!(item.baz(), 3);
    }
    if let Some((_id, item)) = iter.next(){
        assert_eq!(item.bar(), 3);
        assert_eq!(item.baz(), 1); // default value
    }
    Ok(())
}