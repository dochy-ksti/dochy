use crate::{ArcResult, ArchiveOptions, read_archive_data_from_directory, ArchiveOptionsBuilder};
use std::lazy::{SyncLazy};
use std::sync::Arc;
use std::ops::Deref;

pub(crate) static JSON_ARC_OPT : SyncLazy<ArchiveOptions> = SyncLazy::new(|| {
    ArchiveOptionsBuilder::new()
        .add_extension("json5")
        .archive_subfolders(false)
        .build().unwrap()
});

#[test]
fn archive_test() -> ArcResult<()>{
    let archive_data = read_archive_data_from_directory(
        "./src/json/simple",
        Arc::new(|_slice| ()),
        &JSON_ARC_OPT.deref().clone(),
    )?;
    for (path, _) in archive_data.btree(){
        println!("{}", path);
    }
    Ok(())
}

//最後にだけflushするようにしたところさらに少し縮まった

//first_len 62
//len 10 veclen() 62
//len 7 veclen() 62
//len 31 veclen() 62
//len 5449 veclen() 62
//veclen() 3028

//毎回encoderをnewすると多少ファイルサイズが膨らむので、(そもそもencoderの初期化処理も入ってしまうし)
//一気にやったほうがよろしいようだ
//小さいデータは圧縮すると大きくなるが、やはりある程度大きければ縮むようだ

//first_len 62
//len 10 veclen() 90
//len 7 veclen() 105
//len 31 veclen() 144
//len 5449 veclen() 3052

//first vec len 62
//len 10 vec.len 90
//len 7 vec.len 115
//len 31 vec.len 164
//len 5449 vec.len 3082