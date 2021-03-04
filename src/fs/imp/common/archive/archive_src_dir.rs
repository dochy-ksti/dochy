use std::path::{Path};
use crate::core::structs::RootObject;
use crate::archiver::{create_archive_from_directory, CreateArchiveFromDirectory};
use crate::fs::error::FsResult;
use crate::fs::imp::common::path::hash_dir_path::hash_dir_path;
use crate::fs::imp::common::archive::find_or_make_and_read_archive::{find_or_make_and_read_archive, BufOrPath};
use crate::fs::imp::common::archive::read_archive::read_archive;
use crate::fs::common::JSON_ARC_OPT;

/// archiveがない場合archiveを作るのだが、hash値に対応するフォルダが既にある場合は、
/// archiveも既にあるはずなので、archive作成をやめる
/// そこにあるはずのarchiveを探し、そこで見つからない場合、改めて作って読み出す
pub(crate) fn archive_src_dir(src_dir : &Path, proj_dir: &Path) -> FsResult<(RootObject, u128)> {
    let mut buf : Vec<u8> = vec![];

    let r = create_archive_from_directory(
        src_dir, &mut buf,
        |src_hash| {
            hash_dir_path(proj_dir, src_hash).exists()
        }, &*JSON_ARC_OPT)?;
    match r {
        CreateArchiveFromDirectory::Canceled(src_hash) => {
            let archive = find_or_make_and_read_archive(proj_dir, src_hash, BufOrPath::SrcDir(src_dir))?;
            Ok((read_archive(&archive, false)?, src_hash))
        },
        CreateArchiveFromDirectory::WrittenSuccessfully(_, hash) => {
            let archive = find_or_make_and_read_archive(proj_dir, hash, BufOrPath::Buf(&buf))?;
            Ok((read_archive(&archive, false)?, hash))
        }
    }
}

