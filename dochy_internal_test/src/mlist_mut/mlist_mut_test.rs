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
    let mut a1 = huga.first();
    let mb = hoge(&mut a1);
    let a2 = huga.first();
    let h = a2.unwrap();
    Ok(())
}

struct Tekitou<'a, T>{
    item : &'a mut T
}

fn hoge<T>(item : &mut T) -> Tekitou<T>{
    Tekitou{ item }
}

#[test]
fn mlilst_mut_test_another() -> DpResult<()> {
    let old = json_dir_to_root("src/mlist_mut/json", true)?;

    let mut r = RootIntf::new(old);
    let aa = r.mlist();
    let mut huga  = MListMut::new(aa, &mut r);
    {
        let mut a1 = huga.another_first();
    }
    //let mb = hoge(&mut a1);
    {
        let a2 = huga.another_first();

        let h = a2.unwrap();
    }
    Ok(())
}
