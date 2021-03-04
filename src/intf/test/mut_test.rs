// #[cfg(test)]
// mod tests {
//     use nougami_core::json_dir_to_rust;
//
//     #[test]
//     fn it_works() {
//
//         match json_dir_to_rust("src/json_dir/test/mut", true) {
//             Ok(a) => {
//                 let mut root = crate::intf::compile_test::RootIntf::new(a);
//                 let mut mli = root.mutable_list();
//                 let mut f = mli.first().unwrap();
//                 println!("nakabu {}", f.nakabu());
//                 f.set_nakabu(true);
//                 println!("nakabu {}", f.nakabu());
//                 let mut mi = f.mut_inn_list();
//                 let mut fi = mi.first().unwrap();
//                 println!("im {}", fi.inner_mem());
//                 fi.set_inner_mem(101);
//                 println!("im {}", fi.inner_mem());
//             },
//             Err(e) => { println!("val 1 {}", e.message) }
//         }
//     }
// }