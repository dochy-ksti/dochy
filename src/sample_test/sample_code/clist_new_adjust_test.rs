use dochy_core::{json_dir_to_root, adjust_versions};
use crate::sample_test::sample_code::clist_new_accessor::{RootIntf};
use crate::error::DpResult;

#[test]
fn clilst_new_adjust_test() -> DpResult<()> {
    let old = json_dir_to_root("src/sample_test/sample_code_json/clist_old", true)?;
    let new = json_dir_to_root("src/sample_test/sample_code_json/clist_new", true)?;

    let r = adjust_versions(new, old, true)?;

    let r = RootIntf::new(r);
    let mut iter = r.list().iter();

    assert_eq!(iter.next()?.foo(), 3);
    assert_eq!(iter.next()?.foo(), 4);
    assert_eq!(iter.next()?.foo(), -1);
    assert_eq!(iter.next()?.foo(), 5);
    Ok(())
}