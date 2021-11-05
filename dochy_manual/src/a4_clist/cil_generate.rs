use dochy::error::DpResult;
use dochy::core::structs::RootObject;
use dochy::core::json_dir_to_root;
use dochy::intf::generate_interface;

#[test]
fn cil_generate() -> DpResult<()> {
    // Dochy Src can be converted to RootObject with "json_dir_to_root
    let mut root_obj : RootObject = json_dir_to_root(
        /* dir_path */"src/a4_clist/cil",
        /* validation */ true)?;

    // Validation is useful. You should validate your Dochy Src,
    // but do it twice is meaningless. I think the cost of validation is negligible, though.

    // When "validation" is true, verification process will be more detailed

    // "generate_interface" analyzes RootObject, and generates the source code for handling the object in Rust.
    let ans = generate_interface(&mut root_obj);

    // Creates the source file.
    std::fs::write(
        "src/a4_clist/cil_accessor.rs",
        &ans.to_string(),
    ).unwrap();
    Ok(())
}