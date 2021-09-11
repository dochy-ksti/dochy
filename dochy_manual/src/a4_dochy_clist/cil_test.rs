use dochy::error::DpResult;
use dochy::core::structs::RootObject;
use dochy::core::json_dir_to_root;
use dochy::intf::generate_interface;
use crate::a4_dochy_clist::cil_accessor::RootIntf;

#[test]
fn cil_test() -> DpResult<()> {
    let mut root_obj : RootObject = json_dir_to_root("src/a4_dochy_clist/cil", true)?;

    let root = RootIntf::new(root_obj);
    for item in root.clist().iter(){
        let il = item.inner_list();
        println!("len {}", il.len());
        for item in il.iter(){
            println!("il item name {}", item.name());
        }
    }
    Ok(())
}