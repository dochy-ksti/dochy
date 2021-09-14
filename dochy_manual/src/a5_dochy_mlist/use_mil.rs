use dochy::error::DpResult;
use dochy::core::structs::RootObject;
use dochy::core::json_dir_to_root;
use dochy::intf::generate_interface;
use crate::a5_dochy_mlist::mil_accessor::RootIntf;

#[test]
fn mil_generate() -> DpResult<()> {
    let mut root_obj : RootObject = json_dir_to_root("src/a5_dochy_mlist/mil", true)?;

    let ans = generate_interface(&mut root_obj);
    std::fs::write(
        "src/a5_dochy_mlist/mil_accessor.rs",
        &ans.to_string(),
    ).unwrap();
    Ok(())
}

#[test]
fn mil_test() -> DpResult<()> {
    let root_obj : RootObject = json_dir_to_root("src/a5_dochy_mlist/mil", true)?;

    // RootIntf is the struct created from the source file.
    let root = RootIntf::new(root_obj);

    // Iterates the CList
    for (_id, item) in root.mlist().iter(){

        // Gets the inner list from the item.
        let il = item.inner_list();
        println!("len {}", il.len());

        // Iterates the inner list.
        for (_id, item) in il.iter(){
            println!("il item name {}", item.name());
        }
    }
    Ok(())
}