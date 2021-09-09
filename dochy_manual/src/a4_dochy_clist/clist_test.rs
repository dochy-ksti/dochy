use dochy::error::DpResult;
use dochy::core::structs::RootObject;
use dochy::core::json_dir_to_root;

#[test]
fn clist_generate() -> DpResult<()> {
    let old_root : RootObject = json_dir_to_root("src/a4_dochy_collections/clist", true)?;

    RootIntf
    Ok(())
}