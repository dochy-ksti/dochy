use crate::{HashM, IdentityEqual};
use crate::imp::structs::root_value::RootValue;
use crate::imp::structs::rust_param::RustParam;
use crate::imp::structs::util::set_sabun::{SetSabunError, verify_set_sabun};
use crate::imp::structs::meta_table::MetaTable;
use crate::imp::structs::util::hash_m::HashS;
use std::sync::{Arc, Weak};
use crate::imp::structs::list_sab_value::ListSabValue;
use crate::imp::structs::root_sab_value::RootSabValue;

#[derive(Debug, Clone)]
pub struct RootDefObj{
    def : HashM<String, (usize, RootValue)>,
}

impl RootDefObj{
    pub fn new(def : HashM<String, (usize, RootValue)>) -> RootDefObj{
        RootDefObj{ def }
    }
    pub fn def(&self) -> &HashM<String, (usize, RootValue)>{ &self.def }
    ///DefObjは構築後は変更されないのだが、行きがかり上構築時にmodifyが必要になってしまった
    pub(crate) fn def_mut(&mut self) -> &mut HashM<String, (usize, RootValue)>{ &mut self.def }
    pub fn get(&self, name : &str) -> Option<&RootValue>{ self.def.get(name).map(|(_,v)|v) }
    pub fn contains_key(&self, name : &str) -> bool{ self.def.contains_key(name) }
    pub fn len(&self) -> usize{ self.def.len() }
    //pub fn deconstruct(self) -> HashM<String, (usize, RootValue)>{ self.def }
}
