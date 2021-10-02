use std::io::Read;
use crate::{ArchiveData, ArcResult};
use with_capacity_safe::vec_with_capacity_safe;
use std::sync::mpsc::{channel, Receiver, Sender};
use crate::imp::decode::decode;
use crate::imp::structs::archiver::Archiver;
use std::collections::BTreeMap;
use dochy_compaction::kval_enum::KVal;
use std::intrinsics::maxnumf32;


pub(crate) struct ReadItem{
    pub(crate) path : String,
    pub(crate) receiver : Receiver<Vec<u8>>,
}

pub(crate) fn read_items<R : Read, T : Send + 'static>(archiver : Archiver<T>, r : &mut R) -> ArcResult<Vec<(String, Receiver<ArcResult<Vec<u8>>>)>> {
    let (kvals, _) = dochy_compaction::decode(r)?;

    let mut iter = kvals.into_iter();
    let len = iter.next()?.as_i64()? as usize;
    let mut comp_senders: Vec<(usize, Sender<Vec<u8>>)> = vec_with_capacity_safe(len)?;
    let mut comp_receivers: Vec<(Receiver<Vec<u8>>, Sender<ArcResult<Vec<u8>>>)> = vec_with_capacity_safe(len)?;

    let mut raw_receivers: Vec<(String, Receiver<ArcResult<Vec<u8>>>)> = vec_with_capacity_safe(len)?;

    for _ in 0..len {
        let path = iter.next()?.as_string()?;
        let len = iter.next()?.as_i64()? as usize;

        let (comp_sender, comp_receiver) = channel();
        let (raw_sender, raw_receiver) = channel();
        comp_senders.push((len, comp_sender));
        comp_receivers.push((comp_receiver, raw_sender));

        raw_receivers.push((path, raw_receiver));
    }


    std::thread::spawn(move || {
        for (comp_receiver, raw_sender) in comp_receivers{
            rayon::spawn_fifo(move || {
                let mut compressed = comp_receiver.recv().unwrap();
                let mut decoder = snap::raw::Decoder::new();
                match decoder.decompress_vec(&compressed) {
                    Ok(v) => raw_sender.send(Ok(v)).unwrap(),
                    Err(e) => raw_sender.send(Err(e.into())).unwrap(),
                }
            });
        }
    });

    std::thread::spawn(move ||{
        let mut archiver = Archiver::new()
        for item in raw_receivers{

        }
    });
    for (len, sender) in senders {
        let mut buf = vec_with_capacity_safe(len)?;

        r.read_exact(&mut buf)?;
        sender.send()
    }


    Ok(receivers)
}