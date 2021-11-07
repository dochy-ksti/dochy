use std::path::Path;
use dochy::error::DpResult;
use crate::make_manual::make_page::make_page;
use crate::make_manual::write_page::{write_page, write_index_page};
use crate::make_manual::make_index_page::make_index_page;

pub(crate) struct ManualBuilder {
    vec : Vec<ManualBuilderItem>
}

pub(crate) struct ManualBuilderItem {
    title : String,
    src : String
}

impl ManualBuilderItem {
    pub(crate) fn new(title : String, src : String) -> ManualBuilderItem {
        ManualBuilderItem { title, src }
    }
    pub(crate) fn title(&self) -> &str{ &self.title }
    pub(crate) fn src(&self) -> &str{ &self.src }
}

impl ManualBuilder {
    pub(crate) fn new() -> ManualBuilder { ManualBuilder { vec : vec![] } }
    pub(crate) fn add(&mut self, title : String, src : String){
        self.vec.push(ManualBuilderItem::new(title, src))
    }

    pub(crate) fn build<P : AsRef<Path>>(&self, manual_dir : P) -> DpResult<()>{
        let manual_dir = manual_dir.as_ref();

        let vec = &self.vec;
        for i in 0..vec.len(){
            let item = vec.get(i)?;
            if item.src.len() != 0 {
                let prev = get_src(vec, i.overflowing_sub(1).0);
                let next = get_src(vec, i + 1);

                let page = make_page(prev, next, item.title(), item.src())?;
                write_page(item.src(), &page, manual_dir)?;
            }
        }
        let index_page = make_index_page(vec)?;
        write_index_page(&index_page, manual_dir);
        Ok(())
    }
}

fn get_src(vec : &[ManualBuilderItem], index : usize) -> Option<&str>{
    vec.get(index).and_then(|item|
        if item.src.len() == 0 {
            None
        } else{
            Some(item.src.as_str())
        })
}