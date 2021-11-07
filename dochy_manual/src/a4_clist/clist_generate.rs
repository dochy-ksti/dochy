use dochy::error::DpResult;
use dochy::intf::generate_accessor_from_json_dir;

#[test]
fn clist_generate() -> DpResult<()> {
    generate_accessor_from_json_dir("src/a4_clist/clist", "src/a4_clist/clist_accessor.rs", true)?;
    Ok(())
}