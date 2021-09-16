use dochy::error::DpResult;
use dochy::core::structs::RootObject;
use dochy::core::json_dir_to_root;
use dochy::intf::generate_interface;
use crate::a4_clist::cil_accessor::RootIntf;

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

#[test]
fn cil_test() -> DpResult<()> {
    let root_obj : RootObject = json_dir_to_root("src/a4_clist/cil", false)?;
    // Running Validation twice is meaningless, so we didn't do it this time.

    // RootIntf is the wrapper object created from the source file.
    // We can easily access data with it.
    let root = RootIntf::new(root_obj);

    // Iterates the CList
    for item in root.clist().iter(){

        // Gets the inner list from the item.
        let il = item.inner_list();
        println!("len {}", il.len());
        // When inner_list is omitted in the source, len == 0

        // Iterates the inner list.
        for item in il.iter(){
            println!("item name is {}", item.name());
        }
    }
    Ok(())
}