use dochy::error::DpResult;
use dochy::core::structs::RootObject;
use dochy::core::json_dir_to_root;
use dochy::intf::generate_interface;

#[test]
fn save_1gb_generate() -> DpResult<()> {
    let mut root_obj : RootObject = json_dir_to_root("src/save_1gb_100/src_dir", true)?;

    let ans = generate_interface(&mut root_obj);
    std::fs::write(
        "src/save_1gb_100/save_1gb_accessor.rs",
        &ans.to_string(),
    ).unwrap();
    Ok(())
}