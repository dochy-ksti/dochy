use dochy::error::DpResult;
use dochy::core::structs::{RootObject, NullOr, UndefOr, Qv};
use dochy::core::json_dir_to_root;
use dochy::intf::generate_interface;
use crate::a3_dochy_langs_basics::dochy_params_accessor::RootIntf;

#[test]
fn dochy_params_generate() -> DpResult<()> { // DpResult can handle every error type of Dochy

    // You can convert Dochy Src into RootObject with json_dir_to_root
    // RootObject is the object representation of Dochy Src
    let mut root_obj : RootObject = json_dir_to_root(
        /* dir_path */ "src/a3_dochy_langs_basics/dochy_params",
        /* validation */ true)?;

    // Validation is useful. You should validate your Dochy Src,
    // but do it twice is meaningless. I think the cost of validation is negligible, though.

    // "json_dir_to_root" returns CoreResult, which is the result type of the module "dochy_core".
    // It's automatically converted to DpResult with the "?" operator.
    // In Dochy, every result type is automatically converted to DpResult,
    // so basically, users of this library only need DpResult.

    // "generate_interface" analyzes RootObject and generate the source code to access the RootObject in Rust
    let source_file = generate_interface(&mut root_obj);

    // writes the source file as a Rust source file.
    std::fs::write(
        "src/a3_dochy_langs_basics/dochy_params_accessor.rs",
        &source_file.to_string(),
    ).unwrap();

    // Generating source code in [test] code is not sophisticated, but easy. I like it.
    Ok(())
}
