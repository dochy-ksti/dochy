use dochy::core::{json_dir_to_root, adjust_versions};
use dochy::error::DpResult;
use crate::b3_1_clist_and_mlist::mlist_new_accessor::RootIntf;

#[test]
fn mlilst_new_adjust_test() -> DpResult<()> {
    let old = json_dir_to_root("src/b3_1_clist_and_mlist/jsons/mlist_old", true)?;
    let new = json_dir_to_root("src/b3_1_clist_and_mlist/jsons/mlist_new", true)?;

    let r = adjust_versions(new, old, true)?;

    let r = RootIntf::new(r);
    let list = r.mlist();
    let mut iter = list.iter();

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