// use dochy::core::json_dir_to_root;
// use crate::sample_test::sample_code::rpg2_accessor::{RootIntf, ItemsEnum};
// use dochy::error::DpResult;
// use crate::sample_test::sample_code::rpg2_accessor_wrapper::ItemUtil;
//
//
// fn rpg2_test() -> DpResult<()> {
//     let r = json_dir_to_root("src/sample_test/sample_code_json/rpg2", true)?;
//
//     let mut r = RootIntf::new(r);
//
//     for (_id, mut pc) in r.pc_list().iter(){
//         for (_id, item) in pc.items().iter(){
//             format!("the price is {}", item.price());
//             match item.get_enum(){
//                 ItemsEnum::Sword(s) =>{
//                     format!("the attack power is {}", s.attack());
//                 },
//                 ItemsEnum::Herb(h) =>{
//                     format!("the heal power is {}", h.restore());
//                 }
//             }
//         }
//     }
//
//     Ok(())
// }