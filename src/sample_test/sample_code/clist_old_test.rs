use dochy_core::json_dir_to_rust;
use crate::sample_test::sample_code::clist_old_accessor::RootIntf;
use crate::error::DpResult;

#[test]
fn clilst_old_test() -> DpResult<()> {
    let old = json_dir_to_rust("src/sample_test/sample_code_json/clist_old", true)?;

    let r = RootIntf::new(old);
    let mut iter = r.list().iter();

    assert_eq!(iter.next()?.foo(), 1);
    assert_eq!(iter.next()?.foo(), 2);
    assert_eq!(iter.next()?.foo(), 0);
    Ok(())
}