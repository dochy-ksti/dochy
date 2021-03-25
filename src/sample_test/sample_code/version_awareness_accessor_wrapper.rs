use crate::sample_test::sample_code::version_awareness_accessor::RootIntf;
use dochy_core::structs::{UndefOr, NullOr};
use std::cell::UnsafeCell;

pub(crate) struct VeraAccessorWrapper{
    cell : UnsafeCell<RootIntf>
}

impl VeraAccessorWrapper {
    pub fn new(root: RootIntf) -> VeraAccessorWrapper { VeraAccessorWrapper { cell: UnsafeCell::new(root) } }

    // We need to modify the object to get and store the adjusted value,
    // but using a mutable reference to get a value is frustrating,
    // so we use Unsafecell here to make the reference immutable.
    // (Of course you can use &mut self if you want)
    pub fn new_value(&self) -> i64 {
        // get a mutable reference over an immutable &self using UnsafeCell
        let root = unsafe { &mut *self.cell.get() };

        Self::new_value_impl(root)
    }

    fn newest_default_value(root: &RootIntf) -> i64 {
        root.new_value_def_val().into_value().unwrap()
    }

    fn old_value_impl(root: &mut RootIntf) -> i64 {
        match root.old_value() {
            //When the default value is set by hand, we need to distinguish it from the unset default value
            //We changed the default to null, so we can tell unset default value
            NullOr::Null => Self::newest_default_value(root),
            NullOr::Val(v) => {
                root.set_new_value(UndefOr::Val(v * 10));
                root.new_value().into_value().unwrap()
            }
        }
    }

    fn new_value_impl(root: &mut RootIntf) -> i64 {
        match root.new_value() {
            //If the data is old, new_value will be "undefined" because it's not defined at the time.
            UndefOr::Undefined => {
                Self::old_value_impl(root)
            },
            UndefOr::Val(v) => {
                v
                //if a newer version is defined, this needs to be changed to set the newer variable
            }
        }
    }

    // a sample code when the newer version is defined
    // fn new_value_impl(root : &mut RootIntf) -> i64{
    //     match root.new_value(){
    //         Qv::Undefined =>{
    //             Self::old_value_impl(root)
    //         },
    //         Qv::Val(v) =>{
    //             root.set_newest_value(v * 30);
    //             root.newest_value()
    //         },
    //         Qv::Null =>{
    //              Self::newest_default_value(root)
    //         }
    //     }
    // }
}