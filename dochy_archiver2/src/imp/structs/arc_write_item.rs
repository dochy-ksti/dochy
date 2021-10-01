pub(crate) struct ArcWriteItem {
    pub(crate) path : String,
    pub(crate) compressed : Vec<u8>,
    pub(crate) raw_len : usize,
}