use dochy::core::json_dir_to_root;
use dochy::error::DpResult;
use crate::mlist_mut::mlist_mut_accessor::{RootIntf};

//#[test]
fn mlilst_mut_test() -> DpResult<()> {
    let old = json_dir_to_root("src/mlist_mut/json", true)?;

    let mut r = RootIntf::new(old);
    let mut huga  = r.mlist_mut();
    let mut a1 = huga.first().unwrap();
    let mut inl = a1.in_list_mut();
    let a = inl.first().unwrap().a();
    assert_eq!(a,10);

    assert_eq!(a1.bar(),1);
    let mut inl = a1.in_list_mut();
    let a = inl.first().unwrap().a();
    assert_eq!(a,10);

    Ok(())
}

