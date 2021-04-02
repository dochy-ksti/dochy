use crate::sample_test::sample_code::poly_accessor::{MlistMItem, MlistEnum, ReferentACItem, ReferentBCItem, MlistKind};
use dochy_core::structs::{NullOr};

pub(crate) struct ListItemWrapper{
    item : MlistMItem
}

impl ListItemWrapper {
    pub fn new(item: MlistMItem) -> ListItemWrapper { ListItemWrapper { item } }

    pub fn obtain_enum(&self) -> EnumWrapper{
        match self.item.get_enum(){
            MlistEnum::ReferentA(a_item) =>{ EnumWrapper::RefA(RefAWrapper::new(a_item, self.item)) }
            MlistEnum::ReferentB(b_item) =>{ EnumWrapper::RefB(RefBWrapper::new(b_item, self.item)) }
        }
    }

    /// You can change the Enum value here, but is it necessary?
    pub fn set_enum(&mut self, kind : MlistKind){
        self.item.set_enum(kind)
    }
}

pub(crate) enum EnumWrapper{
    RefA(RefAWrapper),
    RefB(RefBWrapper)
}

pub(crate) struct RefAWrapper{
    a_item : ReferentACItem,
    base : MlistMItem,
}

impl RefAWrapper{
    pub fn new(a_item : ReferentACItem, base : MlistMItem) -> RefAWrapper{ RefAWrapper{ a_item, base } }
    pub fn va(&self) -> i64{
        match self.base.va(){
            NullOr::Null =>{
                self.a_item.va()
            },
            NullOr::Val(v) =>{
                v
            }
        }
    }
    pub fn set_va(&mut self, val : i64){
        self.base.set_va(NullOr::Val(val))
    }
    pub fn vc(&self) -> i64{
        match self.base.vc(){
            NullOr::Null =>{
                self.a_item.vc()
            },
            NullOr::Val(v) =>{
                v
            }
        }
    }
    pub fn set_vc(&mut self, val : i64){
        //the member "vc" is common for ReferentA and ReferentB
        //When you modify vc, and then modify Enum to ReferentB,
        //ReferentB's vc will be also the value set here.

        //Maybe it's undesirable, so you may want to make variable-names unique
        //if you want to change Enum
        self.base.set_vc(NullOr::Val(val))
    }

}

pub(crate) struct RefBWrapper{
    b_item : ReferentBCItem,
    base : MlistMItem,
}

impl RefBWrapper{
    pub fn new(b_item : ReferentBCItem, base : MlistMItem) -> RefBWrapper{ RefBWrapper{ b_item, base } }

    pub fn vb(&self) -> i64{
        match self.base.vb(){
            NullOr::Null =>{
                self.b_item.vb()
            },
            NullOr::Val(v) =>{
                v
            }
        }
    }
    pub fn set_vb(&mut self, val : i64){
        self.base.set_vb(NullOr::Val(val))
    }
    pub fn vc(&self) -> i64{
        match self.base.vc(){
            NullOr::Null =>{
                self.b_item.vc()
            },
            NullOr::Val(v) =>{
                v
            }
        }
    }
    pub fn set_vc(&mut self, val : i64){
        self.base.set_vc(NullOr::Val(val))
    }
}
