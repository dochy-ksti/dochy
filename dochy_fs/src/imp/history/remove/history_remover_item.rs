use std::collections::HashMap;
use crate::imp::history::file_name::file_name_props::FileNameProps;
use crate::imp::history::file_hist::file_history_item::FileHistoryItem;
use crate::history::HistoryRemover;

pub(crate) struct Removable(bool);

pub(crate) struct HistoryRemoverItem<'a>{
    cumulative : bool,
    removable : bool,
    props : &'a FileNameProps,
    children : HashMap<u32, HistoryRemoverItem<'a>>,
}

pub(crate) struct RemoveCueItem<'a>{
    ctl : u32,
    order : &'a [u32],
    order_last : Option<u32>,
}
impl<'a> RemoveCueItem<'a>{
    pub(crate) fn new(ctl : u32, order : &'a [u32], order_last : Option<u32>) -> RemoveCueItem<'a>{
        RemoveCueItem{ ctl, order, order_last }
    }

    pub(crate) fn ctl(&self) -> u32{ self.ctl }
    pub(crate) fn order(&self) -> &'a [u32]{ self.order }
    pub(crate) fn order_last(&self) -> Option<u32>{ self.order_last }

    pub(crate) fn from(props : &'a FileNameProps) -> RemoveCueItem<'a>{
        Self::new(props.control(), props.order(), None)
    }
}

impl<'a> HistoryRemoverItem<'a>{
    pub(crate) fn new(cumulative : bool,
               props : &'a FileNameProps,
               children : HashMap<u32, HistoryRemoverItem<'a>>) -> HistoryRemoverItem<'a>{
        HistoryRemoverItem{ cumulative, removable : true, props, children }
    }

    pub(crate) fn children(&self) -> &HashMap<u32, HistoryRemoverItem<'a>>{
        &self.children
    }

    ///自分自身のRemovableをfalseにし、直近の親のorderを返す
    pub(crate) fn keep(&mut self) -> Option<RemoveCueItem<'a>>{
        if self.removable == false{ return None; }

        self.removable = false;
        if self.cumulative{
            let order_last = self.props.order_last();
            if order_last != 0{
                return Some(RemoveCueItem::new(self.props.control(), self.props.order_base(), Some(order_last - 1)));
            }
        }
        if self.props.order().len() != 1{
            return Some(RemoveCueItem::new(self.props.prev_ctl(), self.props.order_base(), None));
        } else{
            return None;
        }
    }

    pub(crate) fn get_removable_props(&self, r : &mut Vec<&'a FileNameProps>){
        if self.removable{
            r.push(self.props())
        }
        for (_,child) in &self.children{
            child.get_removable_props(r);
        }
    }

    pub(crate) fn from(src : &'a FileHistoryItem, props : &'a FileNameProps, max_phase : usize, cumulative_option : bool) -> HistoryRemoverItem<'a>{
        let src_items = src.items();
        let mut children : HashMap<u32, HistoryRemoverItem> = HashMap::with_capacity(src_items.len());
        let src_children = src.children();

        let cumulative = cumulative_option && max_phase - 1 == props.order().len();

        for (index, props) in src_items{
            if let Some(child) = src_children.get(*props.order_last()) {
                children.insert(*index, HistoryRemoverItem::from(child, props, max_phase, cumulative_option));
            } else{
                children.insert(*index, HistoryRemoverItem::new(cumulative, props, HashMap::new()))
            }
        }


        HistoryRemoverItem::new(cumulative, props,  children)
    }
}
