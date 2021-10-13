use dochy::error::DpResult;
use dochy::fs::common::CurrentSrc;
use dochy::fs::history::{HistoryInfo, HistoryOptions, save_history_file, HistoryOptionsBuilder, CumulativeOptionsBuilder};
use crate::b2_save_history_files::save_history_files_accessor::RootIntf;

#[test]
fn save_history_file_test() -> DpResult<()> {
    let history_dir = "src/b2_save_history_files/history_dir";

    // initialize history_dir
    std::fs::remove_dir_all(history_dir).ok();
    std::fs::create_dir(history_dir).unwrap();

    let src_dir = "src/b2_save_history_files/src_dir";

    // HistoryInfo is needed for save/load Dochy History files
    let info = HistoryInfo::create(history_dir,
                                   CurrentSrc::from_src_dir(src_dir),
                                   HistoryOptionsBuilder::new()
                                       .max_phase(2)
                                       .cumulative(Some(CumulativeOptionsBuilder::new()
                                           .limit_count(Some(3)) //Sets limit_count 3 (default: 10)
                                           .limit_nth(Some(1)))) // Sets limit_nth 1 (default : 1)
                                       .build())?;

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
