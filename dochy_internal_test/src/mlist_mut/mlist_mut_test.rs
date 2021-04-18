use dochy::core::json_dir_to_root;
use dochy::error::DpResult;
use crate::mlist_mut::mlist_mut_accessor::{RootIntf, MlistMItem};
use dochy::core::intf::mlist_mut::{MListMut, MListMutItemTrait };

#[test]
fn mlilst_mut_test() -> DpResult<()> {
    let old = json_dir_to_root("src/mlist_mut/json", true)?;

    let mut r = RootIntf::new(old);
    let hoge = r.mlist();
    let mut huga : MlistMListMut = MListMut::new(hoge, &mut r);
    let hego = huga.first();
    Ok(())
}

pub type MlistMListMut<'a> = MListMut<'a, MlistMItemMut<'a>, RootIntf>;

pub struct MlistMItemMut<'a>{
    item : MlistMItem,
    src : &'a mut RootIntf,
}

impl<'a,T> MListMutItemTrait for MlistMItemMut<'a,T>{
    type PtrItem = MlistMItem;

    fn from<T>(ptr_item: Self::PtrItem, src: &mut MListMut<'_, Self, T>) -> Self where Self: Sized {
        MlistMItemMut{
            item : ptr_item,
            src : src.src()
        }
    }
}