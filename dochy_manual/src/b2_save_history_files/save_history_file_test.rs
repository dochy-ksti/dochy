use dochy::error::DpResult;
use dochy::fs::common::CurrentSrc;
use dochy::fs::history::{HistoryInfo, HistoryOptions, save_history_file};
use crate::b2_save_history_files::save_history_files_accessor::RootIntf;

#[test]
fn save_history_file_test() -> DpResult<()> {
    let history_dir = "src/b2_save_history_files/history_dir";

    // make sure the history_dir exists.
    std::fs::create_dir(history_dir).ok();

    let src_dir = "src/b2_save_history_files/src_dir";

    // HistoryInfo is needed for save/load Dochy History files
    let info = HistoryInfo::create(history_dir,
                                   CurrentSrc::from_src_dir(src_dir),
                                   HistoryOptions::new())?;

    // HistoryOptions is linked to history_dir at the first creation of HistoryInfo with the history_dir
    // You can create HistoryInfo twice, but changing the options is forbidden
    // CurrentSrc is also linked and unchangeable.

    let root = info.clone_src_root();
    let mut root = RootIntf::new(root);

    for _ in 0..10{
        root.message_mut().push('a');
        save_history_file(&info, None, root.root_obj_ref())?;
    }


    Ok(())
}
