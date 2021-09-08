use dochy::error::DpResult;
use dochy::core::structs::RootObject;
use dochy::core::json_dir_to_root;
use dochy::intf::generate_interface;

#[test]
fn dochy_params_generate() -> DpResult<()> {
    let mut root_obj : RootObject = json_dir_to_root("src/a3_dochy_langs_basics/dochy_params", true)?;

    let ans = generate_interface(&mut root_obj);
    std::fs::write(
        "src/a3_dochy_langs_basics/dochy_langs_accessor.rs",
        &ans.to_string(),
    ).unwrap();
    Ok(())
}