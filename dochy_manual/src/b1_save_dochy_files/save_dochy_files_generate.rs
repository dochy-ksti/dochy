use dochy::error::DpResult;
use dochy::core::structs::RootObject;
use dochy::core::json_dir_to_root;
use dochy::intf::generate_interface;

#[test]
fn save_dochy_files_generate() -> DpResult<()> {
    let mut root_obj : RootObject = json_dir_to_root("src/b1_save_dochy_files/src_dir", true)?;

    let ans = generate_interface(&mut root_obj);
    std::fs::write(
        "src/b1_save_dochy_files/save_dochy_files_accessor.rs",
        &ans.to_string(),
    ).unwrap();
    Ok(())
}