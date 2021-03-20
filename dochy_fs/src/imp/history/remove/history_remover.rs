use std::collections::HashMap;
use crate::imp::history::file_hist::file_history::FileHistory;
use crate::imp::history::remove::history_remover_item::HistoryRemoverItem;
use crate::imp::history::file_name::file_name_props::FileNameProps;

/// Determines if files are safe to be removed
pub struct HistoryRemover<'a>{
    ctls : HashMap<u32, HistoryRemoverItem<'a>>
}


impl<'a> HistoryRemover<'a>{

    /// Set flags to keep the file and its dependencies
    pub fn keep(&mut self, props : &'a FileNameProps){
        if let Some(ctl)  = self.ctls.get_mut(&props.control()){
            ctl.keep(props.order());
        }
    }

    /// find all the files which can be removed
    pub fn get_removable_props(&self) -> Vec<&'a FileNameProps>{
        let mut vec : Vec<&FileNameProps> = vec![];
        for (_,ctl) in &self.ctls{
            ctl.get_removable_props(&mut vec);
        }
        vec
    }

    pub fn from(history : &'a FileHistory) -> HistoryRemover<'a>{
        let src_ctls = history.ctls();
        let mut r_ctls : HashMap<u32, HistoryRemoverItem> = HashMap::with_capacity(src_ctls.len());
        for (index, ctl) in src_ctls{
            r_ctls.insert(*index, HistoryRemoverItem::from(ctl));
        }
        HistoryRemover{ ctls : r_ctls }
    }


}

