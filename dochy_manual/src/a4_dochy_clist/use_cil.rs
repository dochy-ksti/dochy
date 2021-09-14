use dochy::error::DpResult;
use dochy::core::structs::RootObject;
use dochy::core::json_dir_to_root;
use dochy::intf::generate_interface;
use crate::a4_dochy_clist::cil_accessor::RootIntf;

#[test]
fn cil_generate() -> DpResult<()> {
    // Dochy Src can be converted to RootObject with "json_dir_to_root
    let mut root_obj : RootObject = json_dir_to_root(
        /* dir_path */"src/a4_dochy_clist/cil",
        /* validation */ true)?;

    // When "validation" is true, verification process will be more detailed

    // "generate_interface" analyzes RootObject, and generates the source code for handling the object in Rust.
    let ans = generate_interface(&mut root_obj);

    // Creates the source file.
    std::fs::write(
        "src/a4_dochy_clist/cil_accessor.rs",
        &ans.to_string(),
    ).unwrap();
    Ok(())
}

#[test]
fn cil_test() -> DpResult<()> {
    let root_obj : RootObject = json_dir_to_root("src/a4_dochy_clist/cil", true)?;

    // RootIntf is the struct created from the source file.
    let root = RootIntf::new(root_obj);

    // Iterates the CList
    for item in root.clist().iter(){

        // Gets the inner list from the item.
        let il = item.inner_list();
        println!("len {}", il.len());

        // Iterates the inner list.
        for item in il.iter(){
            println!("il item name {}", item.name());
        }
    }
    Ok(())
}