pub(crate) struct IndexBuilder{
    vec : Vec<IndexBuilderItem>
}

struct IndexBuilderItem{
    title : String,
    src : String
}

impl IndexBuilder{
    pub(crate) fn new() -> IndexBuilder{ IndexBuilder{ vec : vec![] } }
    pub(crate) fn add(&mut self, title : String, src : String){
        self.vec.push(IndexBuilderItem::new(title, src))
    }
}

impl IndexBuilderItem{
    pub(crate) fn new(title : String, src : String) -> IndexBuilderItem{
        IndexBuilderItem{ title, src }
    }
}