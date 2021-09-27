pub struct ArchiveData<T>{
    btree : BTreemap<String, ArchiveDataItem<T>>,
    hash : u128,
}

pub struct ArchiveDataItem<T>{
    converted_data : T,
    compressed_data : Vec<u8>,
}