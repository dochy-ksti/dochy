use crate::{HashM, IdentityEqual};
use crate::imp::structs::root_value::RootValue;
use crate::imp::structs::rust_param::RustParam;
use crate::imp::structs::util::set_sabun::{SetSabunError, verify_set_sabun};
use crate::imp::structs::meta_table::MetaTable;
use crate::imp::structs::util::hash_m::HashS;
use crate::imp::structs::rust_identity::RustIdentity;

#[derive(Debug)]
pub struct RootObject{
    ///listのobjectの場合、defaultはlist側にあるが、ここには初期値が入る。
    default : Box<HashM<String, (usize, RootValue)>>,
    ///変更されたものを記録
    ///listの変更はMutListが直接上書きされるので、sabunには入らない。よってparamだけ記録される
    sabun : Box<HashM<String, RustParam>>,

    ///oldに設定されたメンバは、_Oldを付けなければプログラムから使用できず、
    ///ConstTableである場合、jsonで Refできない
    old : Box<HashS<String>>,

    meta_table : Box<MetaTable>,

    id : RustIdentity,
}

// impl PartialEq for RootObject{
//     fn eq(&self, other: &Self) -> bool {
//         //meta tableは既存の情報を整理しただけで情報量的に変わりがないから等価性には関わらない
//         self.default == other.default && self.sabun == other.sabun && self.old == other.old
//     }
// }

// impl Clone for RootObject{
//     fn clone(&self) -> Self {
//         let default = self.default.clone();
//         let sabun = self.sabun.clone();
//         let old  = self.old.clone();
//         let meta_table = MetaTable::from_root(default.as_ref());
//         Self{ default, sabun, old, meta_table : Box::new(meta_table) }
//     }
// }

impl RootObject{
    pub fn new(default : HashM<String, (usize, RootValue)>, sabun : HashM<String, RustParam>, old : HashS<String>) -> RootObject{
        let meta_table = MetaTable::from_root(&default);
        RootObject{ default: Box::new(default), sabun : Box::new(sabun), old : Box::new(old), meta_table : Box::new(meta_table), id : RustIdentity::new() }
    }
    pub fn default(&self) -> &HashM<String, (usize, RootValue)>{ self.default.as_ref() }

    pub fn meta_table(&self) -> &MetaTable{ self.meta_table.as_ref() }

    ///mlistがdefaultにある都合上、書き換える必要性が生じている。HashMのKeyはmetatableからポインタ参照されているので、ハッシュ再構成が起きてはならない
    pub(crate) fn default_mut(&mut self) -> &mut HashM<String, (usize, RootValue)>{ self.default.as_mut() }

    pub fn mut_refs(&mut self) -> (&mut HashM<String, (usize, RootValue)>,
                                   &mut HashM<String, RustParam>,
                                   &mut HashS<String>, &mut MetaTable){
        (self.default.as_mut(), self.sabun.as_mut(), self.old.as_mut(), self.meta_table.as_mut())
    }

    pub fn deconstruct(self)
        -> (Box<HashM<String, (usize, RootValue)>>, Box<HashM<String, RustParam>>,
            Box<HashS<String>>, Box<MetaTable>){
        (self.default, self.sabun, self.old, self.meta_table)
    }
    pub fn construct(default : Box<HashM<String, (usize, RootValue)>>,
                     sabun : Box<HashM<String, RustParam>>,
                     old : Box<HashS<String>>,
                     meta_table : Box<MetaTable>) -> RootObject{
        RootObject{ default, sabun, old, meta_table, id : RustIdentity::new() }
    }
    pub fn sabun(&self) -> &HashM<String, RustParam>{ self.sabun.as_ref() }
    pub fn sabun_mut(&mut self) -> &mut HashM<String, RustParam>{ self.sabun.as_mut() }
    pub(crate) fn old(&self) -> &HashS<String>{ self.old.as_ref() }
    pub fn set_sabun(&mut self, name : String, param : RustParam) -> Result<Option<RustParam>, SetSabunError> {
        let (p, vt) = if let Some((_,RootValue::Param(p, vt))) = self.default().get(&name) { (p, vt) } else {
            return Err(SetSabunError::ParamNotFound);
        };
        verify_set_sabun(p, vt, &param)?;
        Ok(self.sabun.insert(name, param))
    }

    /// Use some shortcuts to compare data
    /// If the type is the same(created from the same source JSON5),
    /// returning false results is near impossible
    pub fn contents_eq(&self, other : &Self) -> bool{
         self.default().identity_eq(other.default()) &&
             self.sabun().identity_eq(other.sabun())
    }

    pub fn identity(&self) -> &RustIdentity{
        &self.id
    }
}

impl IdentityEqual for (usize, RootValue){
    fn identity_eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1.identity_eq(&other.1)
    }
}
