use dochy::error::DpResult;
use dochy::core::structs::RootObject;
use dochy::core::json_dir_to_root;
use dochy::intf::generate_interface;

#[test]
fn clist_generate() -> DpResult<()> {
    let mut root_obj : RootObject = json_dir_to_root("src/a4_dochy_collections/clist", true)?;

    let ans = generate_interface(&mut root_obj);
    std::fs::write(
        "src/a4_dochy_collections/clist_accessor.rs",
        &ans.to_string(),
    ).unwrap();
    Ok(())
}