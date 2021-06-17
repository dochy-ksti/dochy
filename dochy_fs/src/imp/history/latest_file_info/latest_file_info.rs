use once_cell::sync::Lazy;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use parking_lot::{Mutex, MutexGuard};
use std::sync::Weak;
use crate::history::FileNameProps;

pub struct LatestFileInfo{
    latest_root_id : Weak<()>,
    latest_base_file : FileNameProps,
}

impl LatestFileInfo{
    pub fn new(latest_root_id : Weak<()>, latest_base_file : FileNameProps) -> LatestFileInfo{
        LatestFileInfo{ latest_root_id, latest_base_file }
    }

    pub fn latest_root_id(&self) -> &Weak<()>{ &self.latest_root_id }
    pub fn latest_base_file(&self) -> &FileNameProps{ &self.latest_base_file }
}

pub fn get_mutex<'a, P : AsRef<Path>>(history_hash_dir_path : P, hash : u128) -> &'a Mutex<Option<LatestFileInfo>>{
    let path = history_hash_dir_path.as_ref();
    static ST : Lazy<Mutex<Vec<(PathBuf, u128, Box<Mutex<Option<LatestFileInfo>>>)>>> = Lazy::new(||{
        Mutex::new(Vec::new())
    });
    let mut vec = ST.lock();
    //パスの比較は、多少の表記ゆれなら吸収できる（何も信頼は出来ないが・・・)
    if vec.iter().any(|(p,h, _)| *h == hash && p == path) == false{
        vec.push((path.to_path_buf(), hash, Box::new(Mutex::new(None))));
    }
    let (_,_,b) =vec.iter().find(|(p,h,_)| *h == hash && p == path).unwrap();

    //最初のMutexのlockが解除されるように、vecへの参照をとらず中のMutexだけもらいたい。
    //Boxed ptrなので移動することはないし、削除ルーチンがないので削除されることもないから、ポインタにしてもOKなはずである
    let ptr : *const _ = &**b;

    return unsafe{ &*ptr }
}

pub fn get_latest_file_info<'a, P : AsRef<Path>>(history_dir_path : P, hash : u128) -> MutexGuard<'a, Option<LatestFileInfo>>{
    get_mutex(history_dir_path, hash).lock()
}

pub fn set_latest_file_info<P : AsRef<Path>>(history_dir_path : P, hash : u128, latest_file_info : Option<LatestFileInfo>){
    let mut m = get_mutex(history_dir_path, hash).lock();
    *m = latest_file_info;
}