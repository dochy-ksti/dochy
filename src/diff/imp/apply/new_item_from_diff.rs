use crate::diff::imp::structs_read::ListItemDiffR;
use crate::core::structs::{MetaTables, MutItem, ListSabValue, RefSabValue};
use crate::diff::diff_error::DiffError;
use crate::core::{HashM, HashMt};
use crate::diff::imp::apply::apply_params::apply_params;
use crate::diff::imp::apply::apply_lists::apply_lists;
use crate::diff::imp::apply::apply_refs::apply_refs;

pub(crate) fn new_item_from_diff(diff : ListItemDiffR,
                          meta : &MetaTables) -> Result<MutItem, DiffError>{
    let (params, lists, refs) = diff.deconstruct();
    let mut values : HashM<String, ListSabValue> = HashMt::with_capacity(params.len());
    apply_params(params, meta.items(), &mut values)?;
    apply_lists(lists, meta.items(), &mut values)?;

    let mut ref_hash : HashM<String, RefSabValue> = HashMt::with_capacity(refs.len());
    apply_refs(refs, meta.refs(), &mut ref_hash)?;

    Ok(MutItem::new(values, ref_hash))
}


