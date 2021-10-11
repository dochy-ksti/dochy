use dochy::error::DpResult;
use dochy::core::structs::RootObject;
use dochy::core::json_dir_to_root;
use dochy::intf::generate_interface;

#[test]
fn save_history_files_generate() -> DpResult<()> {
    let mut root_obj : RootObject = json_dir_to_root("src/b2_save_history_files/src_dir", true)?;

    let ans = generate_interface(&mut root_obj);
    std::fs::write(
        "src/b2_save_history_files/save_history_files_accessor.rs",
        &ans.to_string(),
    ).unwrap();
    Ok(())
}