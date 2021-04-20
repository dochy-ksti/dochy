use dochy::core::json_dir_to_root;
use dochy::error::DpResult;
use crate::mlist_mut::mlist_mut_accessor::{RootIntf, MlistMItem};
use dochy::core::intf::mlist_mut::{MListMut };

#[test]
fn mlilst_mut_test() -> DpResult<()> {
    let old = json_dir_to_root("src/mlist_mut/json", true)?;

    let mut r = RootIntf::new(old);
    let aa = r.mlist();
    let mut huga  = MListMut::new(aa, &mut r);
    let mut a1 = huga.first().unwrap();
    huga.first().unwrap().set_bar(10);

    Ok(())
}

