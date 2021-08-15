use once_cell::sync::Lazy;
use std::path::{Path, PathBuf};
use parking_lot::{Mutex, MutexGuard};
use std::sync::Weak;
use crate::history::FileNameProps;

#[derive(Debug, Clone)]
pub struct CurrentRootObjInfo {
    current_root_id: Weak<()>,
    current_base_file: FileNameProps,
    is_newest: bool,
}

impl CurrentRootObjInfo {
    pub fn new(current_root_id: Weak<()>, current_base_file: FileNameProps, is_latest : bool) -> CurrentRootObjInfo {
        CurrentRootObjInfo { current_root_id, current_base_file, is_newest: is_latest }
    }

    pub fn current_root_id(&self) -> &Weak<()>{ &self.current_root_id }
    pub fn current_base_file(&self) -> &FileNameProps{ &self.current_base_file }
    pub fn is_newest(&self) -> bool{ self.is_newest }
}

pub fn get_mutex<'a, P : AsRef<Path>>(history_hash_dir_path : P, hash : u128) -> &'a Mutex<Option<CurrentRootObjInfo>>{
    let path = history_hash_dir_path.as_ref();
    static ST : Lazy<Mutex<Vec<(PathBuf, u128, Box<Mutex<Option<CurrentRootObjInfo>>>)>>> = Lazy::new(||{
        Mutex::new(Vec::new())
    });
    let ptr: *const _ = {
        let mut vec = ST.lock();
        //パスの比較は、多少の表記ゆれなら吸収できる（何も信頼は出来ないが・・・)
        if vec.iter().any(|(p, h, _)| *h == hash && p == path) == false {
            vec.push((path.to_path_buf(), hash, Box::new(Mutex::new(None))));
        }
        let (_, _, b) = vec.iter().find(|(p, h, _)| *h == hash && p == path).unwrap();

        //最初のMutexのlockが解除されるように、vecへの参照をとらず中のMutexだけもらいたい。
        //Boxed ptrなので移動することはないし、削除ルーチンがないので削除されることもないから、ポインタにしてもOKなはずである
        b.as_ref()

    };
    return unsafe{ &*ptr }
}

pub fn get_current_root_obj_info<'a, P : AsRef<Path>>(history_dir_path : P, hash : u128) -> MutexGuard<'a, Option<CurrentRootObjInfo>>{
    let a = get_mutex(history_dir_path, hash).lock();
    a
}

pub fn set_current_root_obj_info<P : AsRef<Path>>(history_dir_path : P, hash : u128, latest_file_info : Option<CurrentRootObjInfo>){
    let mut m = get_mutex(history_dir_path, hash).lock();
    *m = latest_file_info;
}