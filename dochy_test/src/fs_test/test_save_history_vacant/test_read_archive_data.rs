use dochy::error::DpResult;
use dochy_archiver2::{ArchiveData, read_archive_data_from_directory};
use dochy_core::JSON_ARC_OPT;

//#[test]
fn test_read_archive_data() -> DpResult<()>{
    let archive_data : ArchiveData<()> = read_archive_data_from_directory(
        "src/fs_test/test_save_history_vacant/src_dir",
        &*JSON_ARC_OPT,
        |_name, _dat| (),
    )?;
    println!("oked");
    Ok(())
}