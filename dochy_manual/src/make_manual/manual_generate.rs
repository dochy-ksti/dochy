
use dochy::intf::{generate_accessor_from_json_dir};
use dochy::error::DpResult;

#[test]
fn generate_old() -> DpResult<()>{
    generate_accessor_from_json_dir("src/make_manual/manual", "src/make_manual/manual_accessor.rs",true)?;
    Ok(())
}


