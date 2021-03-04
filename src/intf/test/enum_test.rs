// #[cfg(test)]
// mod tests {
//     use nougami_core::json_dir_to_rust;
//     use crate::intf::compile_test::{EnumTestListEnum, EnumTestListKind, Refed1TableID};
//
//     #[test]
//     fn it_works() {
//
//         match json_dir_to_rust("src/json_dir/test/enum", true) {
//             Ok(a) => {
//                 let mut root = crate::intf::compile_test::RootIntf::new(a);
//                 println!("bu {} ", root.bu());
//                 root.set_bu(true);
//                 println!("bu {} ", root.bu());
//                 let mut enum_list = root.enum_test_list();
//                 let f = enum_list.first().unwrap();
//                 match f.get_enum(){
//                     EnumTestListEnum::Refed1(i1) => println!("i1 {}",i1.mem()),
//                     EnumTestListEnum::Refed2(i2) => println!("i2 {}",i2.mem()),
//                 }
//                 f.set_enum(EnumTestListKind::Refed1(Refed1TableID::Second));
//                 match f.get_enum(){
//                     EnumTestListEnum::Refed1(i1) => println!("i1 {}",i1.mem()),
//                     EnumTestListEnum::Refed2(i2) => println!("i2 {}",i2.mem()),
//                 }
//
//             },
//             Err(e) => { println!("val 1 {}", e.message) }
//         }
//     }
// }