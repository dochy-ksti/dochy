use std::collections::BTreeMap;
use crate::imp::history::file_name::file_name_props::FileNameProps;

/// Historyが正しければ、itemの下にchildrenがあるわけだが、誰かが勝手にファイルを削除したりした場合、
/// itemがないのにchildrenだけあるといった事態も起こりうる
#[derive(Debug)]
pub(crate) struct FileHistoryItem{
    children : BTreeMap<u32, FileHistoryItem>,
    items : BTreeMap<u32, FileNameProps>,
}

impl FileHistoryItem{
    pub(crate) fn new() -> FileHistoryItem{
        FileHistoryItem{
            children : BTreeMap::new(),
            items : BTreeMap::new(),
        }
    }

    pub(crate) fn children(&self) -> &BTreeMap<u32, FileHistoryItem>{
        &self.children
    }
    pub(crate) fn items(&self) -> &BTreeMap<u32, FileNameProps>{
        &self.items
    }

    pub(crate) fn insert_or_get_mut(&mut self, order : u32) -> &mut FileHistoryItem {
        if self.children.contains_key(&order) == false{
            let new_his = FileHistoryItem::new();
            self.children.insert(order, new_his);
        }

        self.children.get_mut(&order).unwrap()
    }

    pub(crate) fn insert_props(&mut self, index : u32, props : FileNameProps){
        self.items.insert(index, props);
    }

    pub(crate) fn newest_child(&self) -> Option<(&u32, &FileHistoryItem)>{
        self.children.iter().last()
    }

    pub(crate) fn newest_item(&self) -> Option<(&u32, &FileNameProps)>{
        self.items.iter().last()
    }

    pub(crate) fn get_newest_prop(&self) -> Option<&FileNameProps>{
        let mut his = self;
        loop {
            //dbg!(self.children.len());
            //dbg!(self.items.len());
            if let Some((&item_ind, prop)) = his.newest_item() {
                if let Some((&child_ind, newest_child)) = his.newest_child() {
                    if child_ind < item_ind{
              //          dbg!(format!("child ind {} item ind {}", child_ind, item_ind));
                        return Some(prop);
                    } else {
                //        dbg!("soko");
                        his = newest_child;
                    }
                } else {
                  //  dbg!("koko");
                    return Some(prop);
                }
            }  else{
                //dbg!("empty");
                //historyがemptyの時だけここにくる
                return None;
            }
        }
    }

    pub(crate) fn flatten<'a, 'b>(&'a self, vec : &'b mut Vec<&'a FileNameProps>){
        for (a,b) in &self.items{
            vec.push(b);
            if self.children.contains_key(a){
                self.children[a].flatten(vec);
            }
        }
    }
}