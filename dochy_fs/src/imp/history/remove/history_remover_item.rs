use std::collections::HashMap;
use crate::imp::history::file_name::file_name_props::FileNameProps;
use crate::imp::history::file_hist::file_history_item::FileHistoryItem;

pub(crate) struct Removable(bool);

pub(crate) struct HistoryRemoverItem<'a>{
    items : HashMap<u32, (Removable, &'a FileNameProps)>,
    children : HashMap<u32, HistoryRemoverItem<'a>>,
}
impl<'a> HistoryRemoverItem<'a>{
    pub(crate) fn keep(&mut self, order : &[u32]){
        if order.len() == 0{
            return;
        }
        let ind = order[0];
        if let Some(item) = self.items.get_mut(&ind){
            item.0 = Removable(false);
        }
        if let Some(child) = self.children.get_mut(&ind){
            child.keep(&order[1..]);
        }
    }

    pub(crate) fn get_removable_props(&self, r : &mut Vec<&'a FileNameProps>){
        for (_, (Removable(b), props)) in &self.items{
            if *b{
                r.push(*props);
            }
        }
        for (_,child) in &self.children{
            child.get_removable_props(r);
        }
    }

    pub(crate) fn from(src : &'a FileHistoryItem) -> HistoryRemoverItem<'a>{
        let src_items = src.items();
        let mut r_items : HashMap<u32, (Removable, &FileNameProps)> = HashMap::with_capacity(src_items.len());
        for (index, props) in src_items{
            r_items.insert(*index, (Removable(true), props));
        }
        let src_children = src.children();
        let mut r_children : HashMap<u32, HistoryRemoverItem<'a>> = HashMap::with_capacity(src_children.len());
        for (index, item) in src_children{
            r_children.insert(*index, HistoryRemoverItem::from(item));
        }
        HistoryRemoverItem{ items : r_items, children : r_children }
    }
}
