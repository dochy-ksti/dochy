use crate::diff::imp::read::read_list::read_list;
use crate::core::structs::{MetaValue, MetaTable};
use crate::diff::imp::read::reader::Reader;
use crate::diff::imp::structs_read::ListDiffR;
use crate::diff::diff_error::DiffError;

pub(crate) fn read_lists_numbers(r: &mut Reader, meta: &MetaTable, n: &Vec<usize>,
                      lists: &mut Vec<(usize, Option<ListDiffR>)>) -> Result<(), DiffError>{
    for &id in n {
        if let Some((_, v)) = meta.get(id) {
            match v {
                MetaValue::OptMil(tables) => {
                    if r.read()?.as_bool()? {
                        lists.push((id, Some(read_list(r, tables)?)));
                    } else {
                        lists.push((id, None));
                    }
                },
                MetaValue::MList(tables) => {
                    lists.push((id, Some(read_list(r, tables)?)));
                },
                MetaValue::Param(_) => {}
            }
        }
    }
    Ok(())
}
