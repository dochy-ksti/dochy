use crate::{ArcResult, ArchiveOptions, read_archive_data_from_directory, ArchiveOptionsBuilder, write_archive, read_archive, ArchiveData};
use std::lazy::{SyncLazy};
use std::sync::Arc;
use std::ops::Deref;
use std::sync::mpsc::channel;

pub(crate) static JSON_ARC_OPT : SyncLazy<ArchiveOptions> = SyncLazy::new(|| {
    ArchiveOptionsBuilder::new()
        .add_extension("json5")
        .archive_subfolders(false)
        .build().unwrap()
});

fn hoge<'a, T:Send + 'static>(f : impl Fn(&'a [u8]) -> T + Send + Sync + 'static) -> Arc<dyn Fn(&'a [u8]) -> T + Send + Sync + 'static>{
    Arc::new(f)
}

#[test]
fn archive_test() -> ArcResult<()>{
    let archive_data : ArchiveData<()> = read_archive_data_from_directory(
        "./src/json/simple",
        |_slice| (),
        &JSON_ARC_OPT.deref().clone(),
    )?;
    for (path, _) in archive_data.btree(){
        println!("{}", path);
    }
    let mut buf = Vec::new();
    write_archive(&archive_data, &mut buf)?;
    let read : ArchiveData<()> = read_archive(|_slice| (), &mut buf.as_slice())?;
    for (path, dat) in archive_data.btree(){
        let got = read.btree().get(path)?;
        assert_eq!(got.raw_data(), dat.raw_data());
    }

    Ok(())
}

