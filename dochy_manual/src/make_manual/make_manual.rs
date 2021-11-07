use dochy::error::DpResult;
use dochy::core::json_dir_to_root;
use crate::make_manual::manual_accessor::RootIntf;

#[test]
fn make_manual() -> DpResult<()>{
    let r = json_dir_to_root("src/make_manual/manual", false)?;
    let r = RootIntf::new(r);

    for item in r.list().iter(){
        
    }
    Ok(())
}