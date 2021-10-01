use std::io::Read;
use crate::{ArchiveData, ArcResult};
use with_capacity_safe::vec_with_capacity_safe;
use std::sync::mpsc::{channel, Receiver};
use crate::imp::decode::decode;
use crate::imp::structs::archiver::Archiver;
use std::collections::BTreeMap;
use dochy_compaction::kval_enum::KVal;

pub(crate) fn read_items<R : Read, T : Send + 'static>(archiver : &mut Archiver<T>, r : &mut Read) -> ArcResult<ArchiveData<T>>{
    let (kvals, _) = dochy_compaction::decode(r)?;

    let mut vec : Vec<(String, Receiver<ArcResult<Vec<u8>>>)> = Vec::new();

    let mut iter = kvals.into_iter();
    let len = iter.next()?.as_i64()? as usize;
    for _ in 0..len{
        let path = iter.next()?.as_string()?;
        let (sender, receiver) = channel();

        let bin = if let KVal::Binary(bin) = iter.next()?{
            bin
        } else{
            Err("read failed")?
        };

        let sender = sender.clone();

        rayon::spawn(move||{
            sender.send(decode(&bin));
        });
        vec.push((path, receiver));
    }

    for (path, receiver) in vec{
        let bin = receiver.recv()??;
        archiver.archive(path, bin)
    }

    for (path, comp_len)in vec{
        let mut vec : Vec<u8> = vec_with_capacity_safe(comp_len)?;
        unsafe{ vec.set_len(comp_len); }
        r.read_exact(vec.as_mut_slice());
        let sender = sender.clone();
        rayon::spawn(move||{
            sender.send((path, decode(&vec)));
        });
    }

    let mut btree : BTreeMap<String, Vec<u8>> = BTreeMap::new();
    for _ in 0..vec_len {
        let (path, v) =  receiver.recv().unwrap()?;
        btree.insert(path, v);
    }

    for (path, v) in btree{
        archiver.archive(path, v);
    }

    archiver.finish()
}