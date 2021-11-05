use dochy::core::{json_dir_to_root, adjust_versions};
use dochy::error::DpResult;
use crate::b3_1_clist_and_mlist::undefiable_list_accessor::RootIntf;

#[test]
fn undefiable_list_test() -> DpResult<()> {
    let old = json_dir_to_root("src/b3_1_clist_and_mlist/jsons/no_data", true)?;
    let new = json_dir_to_root("src/b3_1_clist_and_mlist/jsons/undefiable_list_separated", true)?;

    let r = adjust_versions(new, old, true)?;

    let mut r = RootIntf::new(r);
    assert!(r.list().is_none());
    let a = r.list_mut();
    assert_eq!(a.len(), 0);
    Ok(())
}