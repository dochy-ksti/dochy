use dochy::error::DpResult;
use dochy::core::json_dir_to_root;
use crate::make_manual::manual_accessor::RootIntf;
use crate::make_manual::get_content::get_content;
use crate::make_manual::manual_builder::ManualBuilder;

#[test]
fn make_manual() -> DpResult<()>{
    let manual_dir = "manual";
    std::fs::remove_dir_all(manual_dir).ok();
    std::fs::create_dir(manual_dir).unwrap();

    let r = json_dir_to_root("src/make_manual/manual", false)?;
    let r = RootIntf::new(r);
    let mut builder = ManualBuilder::new();

    for item in r.list().iter(){
        builder.add(item.title().to_string(), item.src().to_string());
    }
    builder.build(manual_dir)?;
    Ok(())
}