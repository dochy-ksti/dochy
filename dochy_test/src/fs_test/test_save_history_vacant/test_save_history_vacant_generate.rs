use dochy::error::DpResult;
use dochy::core::structs::RootObject;
use dochy::core::json_dir_to_root;
use dochy::intf::generate_interface;

//#[test]
fn test_save_history_vacant_generate() -> DpResult<()> {
    let mut root_obj : RootObject = json_dir_to_root("src/fs_test/test_save_history_vacant/src_dir", true)?;

    let ans = generate_interface(&mut root_obj);
    std::fs::write(
        "src/fs_test/test_save_history_vacant/test_save_history_vacant_accessor.rs",
        &ans.to_string(),
    ).unwrap();
    Ok(())
}