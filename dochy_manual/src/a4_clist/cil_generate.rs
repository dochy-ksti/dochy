use dochy::error::DpResult;
use dochy::intf::{generate_accessor_from_json_dir};

#[test]
fn cil_generate() -> DpResult<()> {
    generate_accessor_from_json_dir("src/a4_clist/cil", "src/a4_clist/cil_accessor.rs", true)?;
    Ok(())
}