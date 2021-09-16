use dochy::error::DpResult;
use dochy::core::structs::RootObject;
use dochy::core::json_dir_to_root;
use dochy::intf::generate_interface;
use crate::a6_table::table_accessor::RootIntf;


#[test]
fn table_test() -> DpResult<()> {
    let root_obj : RootObject = json_dir_to_root("src/a6_table/table", false)?;

    let mut root = RootIntf::new(root_obj);

    let table1 = root.table1();

    // In tables, you can get items by ID
    println!("item1 val {}", table1.item1().val());

    let a_list = root.a_list();

    for item in a_list.iter(){
        let name = item.name();
        let id = item.ref_id_table1(); // TODO: Stringになってるわよ
        let ref_value = item.ref_table1().val();
    }

    Ok(())
}
