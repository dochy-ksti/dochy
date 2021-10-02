use std::io::Read;
use crate::imp::structs::archiver::Archiver;
use crate::{ArcResult, ArchiveData};

pub(crate) fn read_archive<R : Read, T : Send + 'static>(archiver : &mut Archiver<T>, r : &mut Read) -> ArcResult<ArchiveData<T>>{
    // for (path, receiver) in vec{
    //     let bin = receiver.recv()??;
    //     archiver.archive(path, bin)
    // }
    //
    // for (path, comp_len)in vec{
    //     let mut vec : Vec<u8> = vec_with_capacity_safe(comp_len)?;
    //     unsafe{ vec.set_len(comp_len); }
    //     r.read_exact(vec.as_mut_slice());
    //     let sender = sender.clone();
    //     rayon::spawn(move||{
    //         sender.send((path, decode(&vec)));
    //     });
    // }
    //
    // let mut btree : BTreeMap<String, Vec<u8>> = BTreeMap::new();
    // for _ in 0..vec_len {
    //     let (path, v) =  receiver.recv().unwrap()?;
    //     btree.insert(path, v);
    // }
    //
    // for (path, v) in btree{
    //     archiver.archive(path, v);
    // }

    archiver.finish()
}