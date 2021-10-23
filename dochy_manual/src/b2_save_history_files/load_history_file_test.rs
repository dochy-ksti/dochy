use dochy::error::DpResult;
use dochy::fs::history::{HistoryInfo, HistoryOptionsBuilder, CumulativeOptionsBuilder, load_history_file, list_histories, save_history_file, FileHistories};
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
    //
    // history_dir contains hash_dirs like save_dir.
    // FileHistory is the file list of the hash_dir,
    // and FileHistories is the collection of FileHistory.
    let his : FileHistories = list_histories(&info)?;
    // Gets the list of files from it.
    let files = his.list_files();
    // Finds the file "_0_1_0.his".
    let file = files.iter().find(|file| file.props().calc_filename() == "_0_1_0.his")?;
    // Loads the file.
    let root = load_history_file(&info,
                                 // FileNameProps is the properties got from the filename.
                                 // It represents the dependencies and you can restore the filename precisely from it.
                                 file.props(),
                                 // FileHistory is explained above.
                                 file.history(), false)?;

    let mut root = RootIntf::new(root);
    let mut count = 0;

    modify(&mut root, &mut count);
    // When a RootObject is loaded from a history file, and hasn't been saved before,
    // the save file automatically depends on the loaded file.
    let _file = save_history_file(&info, None, root.root_obj_ref())?;
    // the hash_dir path can be calculated from history_dir and the hash code.
    let hash_dir = hash_dir_path(history_dir, info.hash());
    print_dir(&hash_dir)?;

    // "(0)_1_1_0_0.his 64 bytes" is just created.
    // The first number "(0)" is the control number of the parent.
    // "(0)_1_1_0_0" means "Parent's Control 0 Control 1 Phase-0 1 Phase-1 0 Phase-2 0"
    // The parent of a file can be identified from the filename.
    // The child file's phase numbers except the last is always identical with the parent's, so "Phase-0 1 Phase-1 0".
    // The parent's control is 0, so the parent's numbers are "Control 0 Phase-0 1 Phase-1 0".
    // The filename was "_0_1_0.his", so they are matched.

    // We can't get the parent's tag from the filename, so we can't completely restore the filename of the parent.
    // We need FileHistory to get the filename from the phase numbers and the control number.

    // We can't identify the grandparent because we can't get the grandparent's control number from the filename.
    // We need the parent's filename to identify the grandparent.
    // We can enumerate the ancestors of a file by finding the parent repeatedly.

    // When the parent file is not the latest,
    // or a file is created without a parent, a new control number will be provided to the filename.

    Ok(())
}