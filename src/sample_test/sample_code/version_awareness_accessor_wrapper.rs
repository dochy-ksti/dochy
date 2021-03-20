use crate::sample_test::sample_code::version_awareness_accessor::RootIntf;
use dochy_core::structs::{UndefOr, NullOr};
use std::cell::UnsafeCell;

struct VeraAccessorWrapper{
    int : UnsafeCell<RootIntf>
}

impl VeraAccessorWrapper{
    pub fn new_value(&self) -> i64{
        let int = unsafe{ &mut *self.int.get() };

        match int.new_value(){
            UndefOr::Undefined =>{
                match int.old_value(){
                    NullOr::Null => unreachable!(),
                    NullOr::Val(v) =>{
                        int.set_new_value(UndefOr::Val(v * 10));
                        v * 10
                    }
                }
            },
            UndefOr::Val(v) =>{
                v

            }
        }
    }
}