
use dochy::intf::{generate_accessor_from_json_dir};
use dochy::error::DpResult;

#[test]
fn undefiable_list_generate() -> DpResult<()>{
    generate_accessor_from_json_dir("src/b3_1_clist_and_mlist/jsons/undefiable_list_separated","src/b3_1_clist_and_mlist/undefiable_list_accessor.rs",true)?;
    Ok(())
}