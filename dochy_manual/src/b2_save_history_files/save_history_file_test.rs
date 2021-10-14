use dochy::error::DpResult;
use dochy::fs::common::{CurrentSrc, hash_dir_path};
use dochy::fs::history::{HistoryInfo, save_history_file, HistoryOptionsBuilder, CumulativeOptionsBuilder};
use crate::b2_save_history_files::save_history_files_accessor::RootIntf;
use std::path::Path;

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
                                           .limit_nth(Some(1)))) // Sets limit_nth 1 (default : 1, so this line is not needed)
                                       .build()?)?;

    // HistoryOptions is linked to history_dir at the first creation of HistoryInfo with the history_dir
    // You can create HistoryInfo twice, but changing the options is forbidden
    // CurrentSrc is also linked and unchangeable.

    let root = info.clone_src_root();
    let mut root = RootIntf::new(root);
    let mut count = 0;

    modify(&mut root, &mut count);
    let _file = save_history_file(&info, None, root.root_obj_ref())?;

    let hash_dir = hash_dir_path(history_dir, info.hash());
    print_dir(&hash_dir)?;

    // Let's see what we created.
    //
    // created_time.dat 12 bytes
    // src.archive 96 bytes
    // _0_0.his 15 bytes

    // crated_time.dat and src.archive were explained in the previous section.
    // _0_0.dat is the history file we just created.
    // The first number _0 is what we call "control number". It's appended to make the filename unique.
    // the second _0 is "phase-0 number". It's 0 because it's the first phase-0 file.
    // ".his" is the Dochy History file's extension

    modify(&mut root, &mut count);
    let _file = save_history_file(&info, None, root.root_obj_ref())?;
    print_dir(&hash_dir)?;

    // "_0_0_0.his 20 bytes" is just created.
    // It means "control 0 phase-0 0 phase-1 0"

    modify(&mut root, &mut count);
    let _file = save_history_file(&info, None, root.root_obj_ref())?;
    print_dir(&hash_dir)?;

    // _0_0_0_0.his 24 bytes is just created.
    // It means "control 0 phase-0 0 phase-1 0 phase-2 0"
    // We set max_phase 2 so phase-2 is the max phase.
    // max_phase's total file size is currently 24.
    // limit_nth is "1". and the most largest file in its ancestors is "_0_0_0.his 20 bytes".
    // The total file size of the max phase is larger than the most largest file in its ancestors,
    // so Dochy considers the max phase is already too large.

    modify(&mut root, &mut count);
    let _file = save_history_file(&info, None, root.root_obj_ref())?;
    print_dir(&hash_dir)?;

    // "_0_1.his 25 bytes" is just created. It means "control 0 phase-0 1"
    // When max_phase is overflowed, Dochy calculates which phase should be updated( [algorithm](...) )
    // and Phase-0 is updated.

    Ok(())
}

fn modify(root : &mut RootIntf, count : &mut usize){
    let c = *count % 5;
    *count += 1;
    let m = match c{
        0 => root.d0_mut(),
        1 => root.d1_mut(),
        2 => root.d2_mut(),
        3 => root.d3_mut(),
        4 => root.d4_mut(),
        _ => unreachable!(),
    };
    m.push_str("a");
}

fn print_dir<P : AsRef<Path>>(dir : P) -> DpResult<()>{
    for entry in std::fs::read_dir(dir)?{
        let entry = entry?;
        let name = entry.file_name().to_str()?.to_string();
        let len = entry.metadata()?.len();
        println!("{} {} bytes", name, len);
    }
    Ok(())
}