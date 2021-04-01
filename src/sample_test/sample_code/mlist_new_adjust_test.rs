use dochy_core::{json_dir_to_rust, adjust_versions};
use crate::sample_test::sample_code::mlist_new_accessor::{RootIntf};
use crate::error::DpResult;

#[test]
fn mlilst_new_adjust_test() -> DpResult<()> {
    let old = json_dir_to_rust("src/sample_test/sample_code_json/mlist_old", true)?;
    let new = json_dir_to_rust("src/sample_test/sample_code_json/mlist_new", true)?;

    let r = adjust_versions(new, old, true)?;

    let mut r = RootIntf::new(r);
    let mut iter = r.mlist().iter();

    if let Some((_id, item)) = iter.next(){
        //assert_eq!(item.bar(), 1); //bar is removed
        assert_eq!(item.baz(), 2); //old item's baz is preserved
        assert_eq!(item.qux(), -1); //qux is undefined, so the default value is returned
    }
    if let Some((_id, item)) = iter.next(){
        //assert_eq!(item.bar(), 0);
        assert_eq!(item.baz(), 3); //old item's baz is preserved
        assert_eq!(item.qux(), -1);
    }
    if let Some((_id, item)) = iter.next(){
        //assert_eq!(item.bar(), 0);
        assert_eq!(item.baz(), 100); // This baz was not set in the old data, so the default value of the new version returned
        assert_eq!(item.qux(), -1);
    }
    Ok(())
}