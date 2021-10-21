use dochy::error::DpResult;
use dochy::fs::history::{HistoryInfo, HistoryOptionsBuilder, CumulativeOptionsBuilder, load_history_file, list_histories, save_history_file};
use dochy::fs::common::{CurrentSrc, hash_dir_path};
use crate::b2_save_history_files::save_history_files_accessor::RootIntf;
use crate::b2_save_history_files::save_history_file_test::{modify, print_dir};

pub(crate) fn load_history_file_test() -> DpResult<()>{
    let history_dir = "src/b2_save_history_files/history_dir";
    let src_dir = "src/b2_save_history_files/src_dir";

    let info = HistoryInfo::create(history_dir,
                                   CurrentSrc::from_src_dir(src_dir),
                                   HistoryOptionsBuilder::new()
                                       .max_phase(2)
                                       .cumulative(Some(CumulativeOptionsBuilder::new()
                                           .limit_count(Some(3))))
                                       .build()?)?;

    // The code above is copy-pasted from the previous section.

    // "FileHistories" is needed to load history files.
    // FileHistories is the file list's list of "history_dir".
    // history_dir has "hash_dir"s.
    // The file list of hash_dir is FileHistory. FileHistories is a list of FileHistory.
    //
    // hash_dir is the directory contains history files and a archive of Dochy Src.
    // The hash is calculated from the Dochy Src, and the hash_dir is created with the directory named the hash code.
    // Everytime Dochy Src is modified, a new archive file and a hash_dir is created,
    // and history files created from the Dcohy Src is saved in the hash_dir.
    let his = list_histories(&info)?;
    // Gets the list of file data from it.
    let files = his.list_files();
    // Finds the file "_0_1_0.his".
    let file = files.iter().find(|file| file.props().calc_filename() == "_0_1_0.his")?;
    // Loads the file.
    let root = load_history_file(&info,
                                 // FileNameProps is the propeties gotten from the filename.
                                 // It represents the dependencies and you can restore the filename precisely from it.
                                 file.props(),
                                 // The file list of a hash_dir the file belongs to
                                 file.history(), false)?;

    let mut root = RootIntf::new(root);
    let mut count = 0;

    modify(&mut root, &mut count);
    // When a history file is saved after a loading,
    // it automatically depends on the loaded file.
    let _file = save_history_file(&info, None, root.root_obj_ref())?;
    // the hash_dir path can be calculated from history_dir and the hash code.
    let hash_dir = hash_dir_path(history_dir, info.hash());
    print_dir(&hash_dir)?;

    // "(0)_1_1_0_0.his 64 bytes" is just created.
    // The first number "(0)" is the control number of the parent.
    // The next number "_1" is the control number of this file,
    // so "Control 1 Phase-0 1 Phase-1 0 Phase-2 0"
    // The parent file is always known from the child's filename.
    // The phase numbers except the last is always identical with the parent.
    // The parent's control number is described in the number with "()",
    // so the parent's number is "Control 0 Phase-0 1 Phase-1 0".
    // The filename we opened is "_0_1_0.his", and they are matched.

    // When a history file is derived from a file which is not the latest,
    // or created without a parent, new control number is attached to the new file.

    Ok(())
}