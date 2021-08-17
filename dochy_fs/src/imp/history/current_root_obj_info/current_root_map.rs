use once_cell::sync::Lazy;
use parking_lot::Mutex;
use std::collections::HashMap;
use crate::common::{DochyCache, CurrentSrc};
use crate::history::CurrentRootObjInfo;
use std::sync::atomic::AtomicUsize;
use std::path::{PathBuf, Path};

struct MapItem{
    hash : u128,
    queued : AtomicUsize,
    synced : Box<Mutex<SyncedItem>>
}

pub(crate) struct SyncedItem{
    cache : DochyCache,
    current_root : Option<CurrentRootObjInfo>,
}

static MAP : Lazy<Mutex<HashMap<PathBuf, MapItem>>> = Lazy::new(||{
    Mutex::new(HashMap::new())
});

pub(crate) fn init<P : AsRef<Path>>(history_dir : P, current_src : CurrentSrc){
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
}