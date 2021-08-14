use dochy::error::DpResult;
use dochy::core::structs::RootObject;
use dochy::core::json_dir_to_root;
use dochy::intf::generate_interface;

#[test]
fn hello_history_generate() -> DpResult<()> {
    let mut root_obj : RootObject = json_dir_to_root("src/history_accum_test/src_dir", true)?;

    let ans = generate_interface(&mut root_obj);
    std::fs::write(
        "src/history_accum_test/hello_history_accessor.rs",
        &ans.to_string(),
    ).unwrap();
    Ok(())
}