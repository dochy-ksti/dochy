use std::collections::HashMap;
use crate::imp::history::file_hist::file_history::FileHistory;
use crate::imp::history::remove::history_remover_item::{HistoryRemoverItem, RemoveCueItem};
use crate::imp::history::file_name::file_name_props::FileNameProps;
use crate::imp::history::file_hist::ancestors::create_ancestors_rev;
use crate::error::FsResult;
use crate::imp::history::file_hist::file_history_item::FileHistoryItem;

/// Determines if files are safe to be removed
pub struct HistoryRemover<'a>{
    ctls : HashMap<u32, HistoryRemoverCtlItem<'a>>
}

pub(crate) struct HistoryRemoverCtlItem<'a>{
    ctls : HashMap<u32, HistoryRemoverItem<'a>>
}

impl<'a> HistoryRemover<'a>{

    /// Set flags to keep the file and its ancestors
    pub fn keep(&'a mut self, props : &FileNameProps) -> FsResult<()> {
        let mut cue = RemoveCueItem::from(props);
        let ctl_item = self.ctls.get(&cue.ctl())?;
        loop {
            let mut item = self.get_item_from_cue(&cue)?;
            if let Some(r) = item.keep() {
                cue = r;
            } else {
                break;
            }
        }
        Ok(())
    }



    /// find all the files which can be removed
    pub fn get_removable_props(&self) -> Vec<&'a FileNameProps>{
        let mut vec : Vec<&FileNameProps> = vec![];
        for (_,ctl) in &self.ctls{
            ctl.get_removable_props(&mut vec);
        }
        vec
    }

    pub(crate) fn from(history : &'a FileHistory) -> FsResult<HistoryRemover<'a>>{
        let src_ctls = history.ctls();
        let mut r_ctls : HashMap<u32, HistoryRemoverCtlItem> = HashMap::with_capacity(src_ctls.len());
        for (index, ctl) in src_ctls{
            r_ctls.insert(*index, HistoryRemoverCtlItem::from(ctl, history.max_phase, history.cumulative)?);
        }
        Ok(HistoryRemover{ ctls : r_ctls })
    }
}

impl<'a> HistoryRemoverCtlItem<'a>{
    pub(crate) fn from(ctl : &'a FileHistoryItem, max_phase : usize, cumulative_option : bool) -> FsResult<HistoryRemoverCtlItem<'a>>{
        let items = ctl.items();
        let mut r : HashMap<u32, HistoryRemoverItem> = HashMap::with_capacity(items.len());
        for (index, child) in ctl.children() {
            let props = items.get(index)?;
            r.insert(*index, HistoryRemoverItem::from(ctl, props, max_phase, cumulative_option));
        }
        Ok(HistoryRemoverCtlItem{ ctls : r })
    }

    pub(crate) fn get_item_from_cue(&'a mut self, cue : &RemoveCueItem) -> FsResult<&'a mut HistoryRemoverItem>{
        let mut item = self.ctls.get_mut(cue.order().get(0)?)?;

        for ind in &cue.order()[1..]{
            item = item.children().get_mut(ind)?;
        }

        if let Some(ind) = cue.order_last(){
            item = item.children().get_mut(&ind)?;
        }
        Ok(item)
    }
}
