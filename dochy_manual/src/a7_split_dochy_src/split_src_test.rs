use dochy::core::{json_dir_to_root};
use dochy::error::DpResult;

#[test]
fn split_src_test() -> DpResult<()> {
    let json = json_dir_to_root("src/a7_split_dochy_src/jsons/json", true)?;
    let splitted = json_dir_to_root("src/a7_split_dochy_src/jsons/splitted", true)?;

    assert_eq!(json.default(), splitted.default());
    Ok(())
}