use crate::sample_test::sample_code::version_awareness_accessor::RootIntf;
use crate::core::structs::{UndefOr, NullOr};

struct VeraAccessorWrapper{
    int : RootIntf
}

impl VeraAccessorWrapper{
    pub fn new_value(&mut self) -> i64{
        let int = &mut self.int;

        match int.new_value(){
            UndefOr::Undefined =>{
                match int.old_value(){
                    NullOr::Null => unreachable!(),
                    NullOr::Val(v) =>{
                        int.set_new_value(UndefOr::Val(v * 10))
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