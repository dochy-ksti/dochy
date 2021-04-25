use dochy_core::structs::{RootObject, RootValue, RustParam, MetaValue};
use std::collections::BTreeMap;
use crate::imp::structs_write::{RootDiffW, ListDiffW};
use crate::diff_error::DiffError;
use crate::imp::prepare::get_mlist_diff::get_mlist_diff;
use dochy_core::IdentityEqual;

pub(crate) fn get_root_diff<'a, 'b>(from : &'a RootObject, to : &'b RootObject) -> Result<RootDiffW<'b>, DiffError>{
    let f = from.sabun();
    let t = to.sabun();
    let def = to.default();

    let mut params : BTreeMap<usize, &RustParam> = BTreeMap::new();
    //デフォルト値に書き換えられた場合もsabunからremoveしたりはしないので、
    //書き換えられたことがあるものは必ずsabunにメンバがある。
    //なので変化の可能性があるものは全部toを調べればわかる
    for (key,to_val) in t {
        if let Some(from_val) = f.get(key) {
            if from_val.identity_eq(to_val) == false{
                if let Some((id,_v)) = def.get(key) {
                    params.insert(*id, to_val);
                } else{
                    panic!("invalid def")
                }
            }
        } else{
            if let Some((id,_v)) = def.get(key) {
                params.insert(*id, to_val);
            } else{
                panic!("invalid def")
            }
        }
    }

    let mut lists : BTreeMap<usize, ListDiffW> = BTreeMap::new();
    let from_def = from.default();
    let meta = to.meta_table();
    for (key, (id, value)) in def{
        match value{
            RootValue::MList(to_list) =>{
                if let Some((_, RootValue::MList(from_list))) = from_def.get(key) {
                    if let Some((_key, MetaValue::MList(meta))) = meta.get(*id) {
                        if let Some(list_diff) = get_mlist_diff(from_list.list(), to_list.list(), to_list.default(), meta) {
                            lists.insert(*id, list_diff);
                        }
                    } else{
                        panic!("invalid meta")
                    }
                } else{
                    panic!("invalid from_def")
                }
            },
            _ =>{},
        }
    }
    Ok(RootDiffW::new(params, lists, meta))
}