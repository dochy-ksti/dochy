use dochy::error::DpResult;
use dochy::fs::filesys::{SaveDirInfo, save_dochy_file, list_dochy_files, load_dochy_file};
use dochy::fs::common::CurrentSrc;
use crate::b1_save_dochy_files::save_dochy_files_accessor::RootIntf;

#[test]
fn save_dochy_file_test() -> DpResult<()>
{
    let save_dir = "src/b1_save_dochy_files/save_dir";

    // make sure the save_dir exists.
    std::fs::create_dir(save_dir).ok();

    let src_dir = "src/b1_save_dochy_files/src_dir";

    // You need SaveDirInfo to save Dochy Data files.
    // save_dir and src_dir's paths are needed to create SaveDirInfo.
    let info = SaveDirInfo::create(save_dir, CurrentSrc::from_src_dir(src_dir))?;

    // SaveDirInfo has a RootObject created from "src_dir"

    // We can clone RootObject instantly, because RootObject consists of Arcs (Atomic-Reference-Count Pointer)
    let root = info.clone_src_root();

    let mut root = RootIntf::new(root);
    root.set_message("Hello the next world".to_string());


    let _saved_path = save_dochy_file(
        &info, // SaveDirInfo to specify save_dir, and Dochy Src
        "next_world.dochy", // filename
        root.root_obj_ref(), // the reference of the data. We can get it from "RootIntf::root_obj_ref"
        true)?;

    // SaveDirInfo contains RootObject created from Dochy Src
    // RootIntf contains modified RootObject, and "root_obj_ref()" can get the reference
    // "save_dochy_file" saves the difference of the two.
    // (The difference is "Hello World" -> "Hello the next world")

    // Saving completed. Now let's check out what we just created.

    // save_dir - 80a3e5062f0fbeede35cab8cab5d0826 ┬- created_time.dat
    //                                             ├- next_world.dochy
    //                                             └- src.archive

    // "80a3e5062f0fbeede35cab8cab5d0826" is the hash code created from Dochy Src. It's 128 bit hex value and the algorithm is MetroHash.
    //
    // "src.archive" is the file which archives Dochy Src. Archiving algorithm is original (Dochy Archiver2) and the archiver also calculates the hash.
    // "created_time.dat" is the file to store when this directory is created. (Copying directories often confuses the OS-managed created-time property, so we decided to manage it ourself).
    // "next_world.dochy" is the save file. We designated the filename.

    // Dochy Data (and History) file is placed with "src.archive", which is the Dochy Src the file is originated from.
    // They are placed in a hash-named directory calculated from the Src.
    // When the Src is modified, new hash is calculated and new "src.archive" is created in the new hash-directory and new save files are placed in it.
    // When an old data file is loaded, the data is composed from the correspond old "src.archive" placed in the same directory,
    // and converted to the new version automatically referencing the new version of Dochy Src.

    load_dochy_file_test()?;

    Ok(())
}

fn load_dochy_file_test() -> DpResult<()> {
    let save_dir = "src/b1_save_dochy_files/save_dir";
    let src_dir = "src/b1_save_dochy_files/src_dir";
    let info = SaveDirInfo::create(save_dir, CurrentSrc::from_src_dir(src_dir))?;

    let files = list_dochy_files(save_dir)?;

    // find the file to load.
    let file = files.iter().find(|f| f.name() == "next_world.dochy")?;

    // Dochy Data files are placed in "hash" directory.
    // Let's take a look at the save_dir
    // save_dir - 9b945bdd6f75d246a40f6e11555559cb ┬- created_time.dat
    //                                             ├- next_world.dochy
    //                                             └- src.archive

    let root = load_dochy_file(
        file.calc_path(save_dir),
        &info,
        true
    )?;
    let root = RootIntf::new(root);
    assert_eq!(root.message(), "Hello the next world");

    Ok(())
}