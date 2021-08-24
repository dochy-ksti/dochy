use dochy::error::DpResult;
use dochy::fs::common::{CurrentSrc};
use std::path::{Path, PathBuf};
use dochy::core::structs::RootObject;
use rand::Rng;
use dochy::fs::filesys::SaveDirInfo;

///10メンバのうち1メンバを上書きするので90%はそのままだが、
/// 各メンバ(Binary)がジワジワ伸びていくので全体のファイルサイズは大きくなっていく
/// そんな状態が正しくHistoryとして記録できているか、目視とload and compareで確認
//#[test]
fn test_save_async() -> DpResult<()> {
    let root_dir = Path::new("src/fs_test/test_save_async");
    let save_dir = root_dir.join("save_dir");
    std::fs::create_dir(&save_dir).ok().unwrap();
    let info = SaveDirInfo::create(&save_dir,
                                   CurrentSrc::from_src_dir("src/fs_test/test_save_async/src_dir"))?;

    let root = info.clone_src_root();
    unimplemented!()
}