use dochy::core::json_dir_to_root;
use dochy::error::DpResult;
use crate::mlist_mut::mlist_mut_accessor::{RootIntf};
use dochy::core::intf::mlist_mut::{MListMut };

#[test]
fn mlilst_mut_test() -> DpResult<()> {
    let old = json_dir_to_root("src/mlist_mut/json", true)?;

    let mut r = RootIntf::new(old);
    let aa = r.mlist();
    let mut huga  = MListMut::new(aa, &mut r);
    let mut a1 = huga.first().unwrap();
    let mut inl = MListMut::new(a1.in_list(), &mut a1);
    let a = inl.first().unwrap().a();

    let hoge = a1.bar();
    let mut inl = MListMut::new(a1.in_list(), &mut a1);
    let a = inl.first().unwrap().a();
    println!("{} {}", a, hoge);

    Ok(())
}

