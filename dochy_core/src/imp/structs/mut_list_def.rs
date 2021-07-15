use crate::imp::structs::list_def_obj::ListDefObj;

#[derive(Debug,  Clone)]
pub struct MutListDef {
    list_def : Box<ListDefObj>,
    undefinable: bool,
    //compatible : Box<HashS<String>>,
}

impl MutListDef {
    pub(crate) fn new(list_def : ListDefObj, undefinable : bool) -> MutListDef {
        MutListDef { list_def : Box::new(list_def), undefinable }
    }
    pub fn list_def(&self) -> &ListDefObj{ self.list_def.as_ref() }
    pub fn undefinable(&self) -> bool{ self.undefinable }
    //pub(crate) fn compatible(&self) -> &HashS<String>{ self.compatible.as_ref() }
}

