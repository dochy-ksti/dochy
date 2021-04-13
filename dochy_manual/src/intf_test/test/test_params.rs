// #[cfg(test)]
// mod tests {
//     use nougami_core::json_dir_to_rust;
//     use nougami_core::intf::null_or::{NullOr, UndefOr};
//     use nougami_core::structs::Qv;
//
//     #[test]
//     fn it_works() {
//
//         match json_dir_to_rust("src/json_dir/test/params", true) {
//             Ok(mut a) => {
//                 let mut root_intf = crate::test_generated::params::test::RootIntf::new(a);
//                 let mut intf = root_intf;
//                 assert_eq!(intf.s(),"sutoringu");
//                 assert_eq!(intf.shatena(), NullOr::Val("esuhatena".to_string()));
//                 assert_eq!(intf.sbikkuri(), UndefOr::Undefined);
//                 assert_eq!(intf.sbh(), Qv::Val("bikkurihatena".to_string()));
//                 intf.set_s("s2".to_string());
//                 assert_eq!(intf.s(),"s2");
//                 intf.set_shatena(NullOr::Null);
//                 assert_eq!(intf.shatena(),NullOr::Null);
//                 intf.set_sbikkuri(UndefOr::Val("und".to_string()));
//                 assert_eq!(intf.sbikkuri(),UndefOr::Val("und".to_string()));
//                 intf.set_sbh(Qv::Null);
//                 assert_eq!(intf.sbh(),Qv::Null);
//
//                 assert_eq!(intf.b(),false);
//                 assert_eq!(intf.bhatena(), NullOr::Null);
//                 assert_eq!(intf.bbikkuri(), UndefOr::Val(false));
//                 assert_eq!(intf.bbh(), Qv::Val(true));
//                 intf.set_b(true);
//                 assert_eq!(intf.b(),true);
//                 intf.set_bhatena(NullOr::Val(false));
//                 assert_eq!(intf.bhatena(),NullOr::Val(false));
//                 intf.set_bbikkuri(UndefOr::Undefined);
//                 assert_eq!(intf.bbikkuri(),UndefOr::Undefined);
//                 intf.set_bbh(Qv::Null);
//                 assert_eq!(intf.bbh(),Qv::Null);
//
//                 assert_eq!(intf.int(), 10);
//                 intf.set_int(20);
//                 assert_eq!(intf.int(), 20);
//
//                 assert_eq!(intf.float(), 12.0);
//                 intf.set_float(22.0);
//                 assert_eq!(intf.float(), 22.0);
//
//
//             },
//             Err(e) => { println!("val 1 {}", e.message) }
//         }
//     }
// }