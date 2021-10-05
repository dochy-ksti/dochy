use dochy_archiver2::ArchiveData;
use crate::error::{CoreResult};
use crate::structs::{ RootObject};
use crate::imp::json_to_rust::json_file_to_rust::ArchivingItem;
use crate::imp::json_to_rust::construct_root::construct_root;

pub(crate) fn archive_data_to_root_with_hash(data : ArchiveData<CoreResult<ArchivingItem>>)
    -> CoreResult<(RootObject, u128)> {
    let hash = data.hash();
    let mut tree = data.deconstruct();
    let root: CoreResult<RootObject> = (|| {
        let key =
            if let Some((key, _)) = tree.iter().find(|(_key, val)| {
                if let Ok(ArchivingItem::Root(_)) = val.converted_data() {
                    true
                } else {
                    false
                }
            }) {
                key.to_string()
            } else {
                Err("couldn't find root.json5")?
            };

        let item = tree.remove(&key).unwrap();
        let (item, _) = item.deconstruct();

        if let Ok(ArchivingItem::Root(root)) = item {
            return Ok(root)
        } else {
            unreachable!()
        }
    })();
    let root = root?;

    let mut vec = Vec::with_capacity(tree.len());
    for (_path, val) in tree {
        let (item, _) = val.deconstruct();
        match item {
            Ok(ArchivingItem::Item((name, val, sab))) =>{
                vec.push((name, val, sab));
            }
            Err(e) =>{ return Err(e); }
            _ => { unreachable!() }
        }
    }

    let root = construct_root(root, vec)?;
    return Ok((root, hash));
}