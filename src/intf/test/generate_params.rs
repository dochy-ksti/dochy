// #[cfg(test)]
// mod tests {
//     use nougami_core::json_dir_to_rust;
//     use crate::intf::generate_interface;
//     use crate::intf::test::write_file::write_file;
//     //use crate::intf::rust_to_json_new_default;
//     //use crate::intf::imp::json_to_rust::json_root_to_rust;
//     //use crate::intf::imp::rust_to_json::root_to_json::root_to_json_new_default;
//
//     #[test]
//     fn it_works() {
//         match json_dir_to_rust("src/json_dir/test/params", true) {
//             Ok(a) => {
//                 //println!("{:?}", a);
//                 let ans = generate_interface(&a);
//                 write_file("src/compile_test.rs", &ans.to_test_string());
//             },
//             Err(e) => { println!("val 1 {}", e.message) }
//         }
//     }
// }