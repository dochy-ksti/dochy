use dochy::error::DpResult;
use dochy::core::structs::RootObject;
use dochy::core::json_dir_to_root;
use dochy::intf::generate_interface;
use crate::a3_dochy_langs_basics::dochy_params_accessor::RootIntf;

#[test]
fn dochy_params_generate() -> DpResult<()> {
    // You can convert Dochy Src into RootObject with json_dir_to_root
    // RootObject is the object representation of Dochy Src
    let mut root_obj : RootObject = json_dir_to_root(
        /* dir_path */ "src/a3_dochy_langs_basics/dochy_params",
        /* validation */ true)?;

    // Validation is useful. You should validate your Dochy Src,
    // but do it twice is meaningless. I think the cost of validation is negligible, though.

    // generate_interface analyzes RootObject and generate the source code to access the RootObject in Rust
    let source_file = generate_interface(&mut root_obj);

    // writes the source file as a Rust source file.
    std::fs::write(
        "src/a3_dochy_langs_basics/dochy_params_accessor.rs",
        &source_file.to_string(),
    ).unwrap();

    // Generating source code in [test] code is not sophisticated, but easy. I like it.
    Ok(())
}

#[test]
fn params_test() -> DpResult<()> {
    // You've validated Dochy Src when you generated the Rust source, so validation is not needed now.
    let root_obj : RootObject = json_dir_to_root("src/a3_dochy_langs_basics/dochy_params", false)?;

    // RootIntf is the struct created in the generated source.
    // It's always named RootIntf.
    let mut root = RootIntf::new(root_obj);

    fn typename<T>(a : &T) -> &'static str{
        std::any::type_name::<T>()
    }
    // Let's see what RootIntf does
    let ival = root.int_value();
    let fval  = root.float_value();
    let bool = root.bool_value();

    assert_eq!(typename(&ival), "i64");
    assert_eq!(typename(&fval), "f64");
    assert_eq!(typename(&bool), "bool");

    // i64, f64, and bool are copyable types. They are copied

    let sval : &String = root.string_value();
    let iarray : &Vec<i64> = root.int_array();
    let farray : &Vec<f64> = root.float_array();
    let bin : &Vec<u8> = root.binary();
    assert_eq!(typename(sval), "String");

    // sval is "Str" type.


    Ok(())
}