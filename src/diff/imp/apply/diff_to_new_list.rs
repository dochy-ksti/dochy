use crate::diff::imp::structs_read::ListDiffR;
use crate::diff::diff_error::DiffError;
use crate::core::structs::{MutInnerList, LinkedMap, MutItem, MetaTables};
use crate::diff::imp::apply::apply_list_diff::apply_list_diff;

pub(crate) fn diff_to_new_list(diff : ListDiffR, meta : &MetaTables) -> Result<MutInnerList, DiffError>{
    let mut map : LinkedMap<MutItem> = LinkedMap::new();

    apply_list_diff(&mut map, diff, meta)?;

    Ok(MutInnerList::new(map))
}