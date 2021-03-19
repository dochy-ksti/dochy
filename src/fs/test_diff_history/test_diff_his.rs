use crate::core::intf::RootObjectPtr;
use crate::core::structs::Qv;
use tempfile::tempdir;
use crate::fs::error::FsResult;
use crate::fs::imp::common::current_src::CurrentSrc;
use crate::fs::history::{next, DochyCache, HistoryOptions, HistoryOptionsBuilder, list_histories, CumulativeOptionsBuilder, load_history_file_data};
use crate::core::intf::root::set_int;
use crate::fs::test_fs::copy_dir_all::copy_dir_all;
use crate::core::json_dir_to_rust;
use crate::fs::test_diff_history::show_dir_contents_diff_history::show_dir_contents_diff_history;

//#[test]
fn test_diff_his() -> FsResult<()> {
    let dir = tempdir()?;
    let proj_dir_path = dir.path();
    let src_dir_path = proj_dir_path.join("simple_src");
    let current_src = CurrentSrc::SrcDir(src_dir_path.clone());

    copy_dir_all("src/json_dir/simple", &src_dir_path)?;

    let mut cache = DochyCache::new(current_src.clone(), false, false);
    let opt = HistoryOptions::from(HistoryOptionsBuilder {
        max_phase: 1,
        update_phase_a: true,
        cumulative: Some(CumulativeOptionsBuilder {
            limit_count: Some(3),
            limit_nth: None,
        }),
    })?;

    let mut root = json_dir_to_rust(&src_dir_path, false)?;
    for i in 0..2 {
        let p = RootObjectPtr::new(&mut root);
        set_int(p, "int", Qv::Val(i));

        next(proj_dir_path, None, &root, &mut cache, &opt)?;
        let histories = list_histories(proj_dir_path)?;

        let newest = histories.get_newest_file_data()?;


        let loaded = load_history_file_data(proj_dir_path,
                                            &newest, &mut cache, &opt, false)?;
        assert_eq!(root, loaded);
    }

    std::fs::copy("src/json_dir/simple_mod1/root.json5_back", &src_dir_path.join("root.json5_back"))?;
    let histories = list_histories(proj_dir_path)?;
    let newest = histories.get_newest_file_data()?;

    let mut root = load_history_file_data(proj_dir_path, &newest, &mut cache, &opt, false)?;
    let p = RootObjectPtr::new(&mut root);
    let mut cache = DochyCache::new(current_src.clone(), false, false);

    for i in 0..15 {
        set_int(p, "int", Qv::Val(i));

        next(proj_dir_path, None, &root, &mut cache, &opt)?;
        let histories = list_histories(proj_dir_path)?;

        let newest = histories.get_newest_file_data()?;

        let loaded = load_history_file_data(proj_dir_path,
                                            &newest, &mut cache, &opt, false)?;
        assert_eq!(root, loaded);

        histories.remove_old_files(5, proj_dir_path)?;

        for (hash, name, size) in show_dir_contents_diff_history(proj_dir_path)?{
            println!("{} {} {}", hash, name, size);
        }

        println!();
    }



    Ok(())
}