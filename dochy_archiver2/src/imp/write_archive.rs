use crate::ArchiveData;
use dochy_compaction::kval_enum::KVal;

pub fn write_archive<T : Send + 'static>(data : ArchiveData<T>){
    let mut vec : Vec<KVal> = vec![];

    // for (key,val) in data.btree(){
    //     rayon::spawn(move|| )
    // }

}