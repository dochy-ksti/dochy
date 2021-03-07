//
//
// #[cfg(test)]
// mod tests {
//     use crate::testing::diff::util::get_root_obj::get_root_obj;
//     use crate::testing::diff::generated_test_big_structure::test::RootIntf;
//
//     #[test]
//     fn test_big_structure(){
//         match test_mem1(){
//             Ok(()) =>{},
//             Err(s) => println!("{}", s),
//         }
//
//         match test_mem63_65(){
//             Ok(()) =>{},
//             Err(s) => println!("{}", s),
//         }
//
//     }
//
//
//     #[test]
//     fn test_zero() -> Result<(), String>{
//         let json_dir_path = "src/testing/diff/diff_big_structure/";
//         let root_obj = get_root_obj(json_dir_path)?;
//
//         let intf = RootIntf::new(root_obj);
//
//         let moto = get_root_obj(json_dir_path)?;
//
//         //zero
//         //let diff = crate::diff::get_kvals(&moto, unsafe{ intf.root_obj_ref() }).or_else(|e| Err(e.message))?;
//         //println!("{:?}", diff);
//
//         let diff = crate::diff::get_diff(&moto, unsafe{ intf.root_obj_ref() }).or_else(|e| Err(e.message))?;
//         let applied = crate::diff::apply_diff(moto, diff).or_else(|e| Err(e.message))?;
//         //let intf = RootIntf::new(applied);
//
//         let moto = get_root_obj(json_dir_path)?;
//         assert_eq!(moto, applied);
//
//         Ok(())
//     }
//
//     fn test_mem1() -> Result<(), String>{
//         let json_dir_path = "src/testing/diff/diff_big_structure/";
//         let root_obj = get_root_obj(json_dir_path)?;
//
//         let mut intf = RootIntf::new(root_obj);
//         assert_eq!(intf.mem1(), 2);
//         intf.set_mem1(1);
//
//         let moto = get_root_obj(json_dir_path)?;
//
//         let diff = crate::diff::get_diff(&moto, unsafe{ intf.root_obj_ref() }).or_else(|e| Err(e.message))?;
//         let applied = crate::diff::apply_diff(moto, diff).or_else(|e| Err(e.message))?;
//         let intf = RootIntf::new(applied);
//
//         assert_eq!(intf.mem1(), 1);
//         Ok(())
//     }
//
//     fn test_mem63_65() -> Result<(), String>{
//         let json_dir_path = "src/testing/diff/diff_big_structure/";
//         let root_obj = get_root_obj(json_dir_path)?;
//
//         let mut intf = RootIntf::new(root_obj);
//         intf.set_mem63(1);
//         intf.set_mem64(1);
//         intf.set_mem65(1);
//
//         let moto = get_root_obj(json_dir_path)?;
//
//         //bits
//         //let diff = crate::diff::get_kvals(&moto, unsafe{ intf.root_obj_ref() }).or_else(|e| Err(e.message))?;
//         //println!("{:?}", diff);
//         let diff = crate::diff::get_diff(&moto, unsafe{ intf.root_obj_ref() }).or_else(|e| Err(e.message))?;
//         let applied = crate::diff::apply_diff(moto, diff).or_else(|e| Err(e.message))?;
//         let intf = RootIntf::new(applied);
//
//         assert_eq!(intf.mem63(), 1);
//         assert_eq!(intf.mem64(), 1);
//         assert_eq!(intf.mem65(), 1);
//         Ok(())
//     }
//
//     #[test]
//     fn test_mem64_65() -> Result<(), String>{
//         let json_dir_path = "src/testing/diff/diff_big_structure/";
//         let root_obj = get_root_obj(json_dir_path)?;
//
//         let mut intf = RootIntf::new(root_obj);
//         intf.set_mem64(1);
//         intf.set_mem65(1);
//
//         let moto = get_root_obj(json_dir_path)?;
//
//         //store ids はnumbers
//         // let diff = crate::diff::get_kvals(&moto, unsafe{ intf.root_obj_ref() }).or_else(|e| Err(e.message))?;
//         //println!("{:?}", diff);
//         let diff = crate::diff::get_diff(&moto, unsafe{ intf.root_obj_ref() }).or_else(|e| Err(e.message))?;
//         let applied = crate::diff::apply_diff(moto, diff).or_else(|e| Err(e.message))?;
//         let intf = RootIntf::new(applied);
//
//         assert_eq!(intf.mem64(), 1);
//         assert_eq!(intf.mem65(), 1);
//         Ok(())
//     }
//
//     #[test]
//     fn test_mem58_62() -> Result<(), String>{
//         let json_dir_path = "src/testing/diff/diff_big_structure/";
//         let root_obj = get_root_obj(json_dir_path)?;
//
//         let mut intf = RootIntf::new(root_obj);
//         intf.set_mem58(1);
//         intf.set_mem59(1);
//         intf.set_mem60(1);
//         intf.set_mem61(1);
//         intf.set_mem62(1);
//
//         let moto = get_root_obj(json_dir_path)?;
//
//         //numbers
//         //let diff = crate::diff::get_kvals(&moto, unsafe{ intf.root_obj_ref() }).or_else(|e| Err(e.message))?;
//         //println!("{:?}", diff);
//         let diff = crate::diff::get_diff(&moto, unsafe{ intf.root_obj_ref() }).or_else(|e| Err(e.message))?;
//         let applied = crate::diff::apply_diff(moto, diff).or_else(|e| Err(e.message))?;
//         let intf = RootIntf::new(applied);
//         assert_eq!(intf.mem58(), 1);
//         assert_eq!(intf.mem59(), 1);
//         assert_eq!(intf.mem60(), 1);
//         assert_eq!(intf.mem61(), 1);
//         assert_eq!(intf.mem62(), 1);
//
//         Ok(())
//     }
//
//     #[test]
//     fn test_mem58_63() -> Result<(), String>{
//         let json_dir_path = "src/testing/diff/diff_big_structure/";
//         let root_obj = get_root_obj(json_dir_path)?;
//
//         let mut intf = RootIntf::new(root_obj);
//         intf.set_mem58(1);
//         intf.set_mem59(1);
//         intf.set_mem60(1);
//         intf.set_mem61(1);
//         intf.set_mem62(1);
//         intf.set_mem63(1);
//         //intf.set_mem64(1);
//
//         let moto = get_root_obj(json_dir_path)?;
//
//         //u64
//         //let diff = crate::diff::get_kvals(&moto, unsafe{ intf.root_obj_ref() }).or_else(|e| Err(e.message))?;
//         //println!("{:?}", diff);
//         let diff = crate::diff::get_diff(&moto, unsafe{ intf.root_obj_ref() }).or_else(|e| Err(e.message))?;
//         let applied = crate::diff::apply_diff(moto, diff).or_else(|e| Err(e.message))?;
//         let intf = RootIntf::new(applied);
//         assert_eq!(intf.mem58(), 1);
//         assert_eq!(intf.mem59(), 1);
//         assert_eq!(intf.mem60(), 1);
//         assert_eq!(intf.mem61(), 1);
//         assert_eq!(intf.mem62(), 1);
//         assert_eq!(intf.mem63(), 1);
//         //assert_eq!(intf.mem64(), 1);
//
//         Ok(())
//     }
//
//
//
//
//
// }