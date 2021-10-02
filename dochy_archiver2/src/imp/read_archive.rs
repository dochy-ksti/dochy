use std::io::Read;
use crate::{ArchiveData, ArcResult};
use with_capacity_safe::vec_with_capacity_safe;
use std::sync::mpsc::{channel, Receiver, Sender};
use crate::imp::decode::decode;
use crate::imp::structs::archiver::Archiver;
use std::collections::BTreeMap;
use dochy_compaction::kval_enum::KVal;
use std::intrinsics::maxnumf32;
use std::sync::Arc;


pub(crate) struct ReadItem{
    pub(crate) path : String,
    pub(crate) receiver : Receiver<Vec<u8>>,
}

pub(crate) fn read_archive<R : Read, T : Send + 'static>(converter : Arc<dyn Fn(&[u8]) -> T + Send + Sync>, r : &mut R) -> ArcResult<ArchiveData<T>> {
    let (kvals, _) = dochy_compaction::decode(r)?;

    let mut iter = kvals.into_iter();
    let len = iter.next()?.as_i64()? as usize;
    let mut raw_senders: Vec<(usize, Sender<ArcResult<Vec<u8>>>)> = vec_with_capacity_safe(len)?;
    let mut raw_receivers: Vec<(String, Receiver<ArcResult<Vec<u8>>>)> = vec_with_capacity_safe(len)?;

    for _ in 0..len {
        let path = iter.next()?.as_string()?;
        let len = iter.next()?.as_i64()? as usize;

        let (raw_sender, raw_receiver) = channel();
        raw_senders.push((len, raw_sender));
        raw_receivers.push((path, raw_receiver));
    }

    let (archive_sender, archive_receiver) = channel();

    std::thread::spawn(move ||{
        let mut archiver = Archiver::new(converter);
        for (path, data) in raw_receivers{
            match data.recv().unwrap(){
                Ok(v) =>{ archiver.archive(path, v); },
                Err(e) => {
                    archive_sender.send(Err(e));
                    return;
                }
            }
        }
        archive_sender.send(archiver.finish())
    });
    for (len, raw_sender) in raw_senders {
        let mut buf = vec_with_capacity_safe(len)?;

        r.read_exact(&mut buf)?;
        rayon::spawn_fifo(move ||{
            let mut decoder = snap::raw::Decoder::new();
            match decoder.decompress_vec(&buf) {
                Ok(v) => raw_sender.send(Ok(v)).unwrap(),
                Err(e) => raw_sender.send(Err(e.into())).unwrap(),
            }
        })
    }


    archive_receiver.recv().unwrap()
}