// #[cfg(test)] pub(crate ) mod test{
//     use nougami_core::intf::*;
//     use nougami_core::structs::*;
//
//     pub(crate ) struct RootIntf{
//         root : Box<RootObject>,
//         item : Box<RootItem>,
//     }
//     pub(crate ) struct RootItem{
//         ptr : RootObjectPtr
//     }
//     impl RootIntf{
//         pub(crate ) fn new(obj : RootObject) -> RootIntf{
//             let root = Box::new(obj);
//             let item = Box::new(RootItem{ ptr : RootObjectPtr::new(root.as_ref()) });
//             RootIntf{ root, item }
//         }
//         pub(crate ) fn ptr(&self) -> RootObjectPtr{ self.item.as_ref().ptr }
//
//         pub(crate ) fn bbikkuri(&self) -> UndefOr<bool>{
//             let qv = root::get_bool(self.ptr(), "bbikkuri").unwrap();
//             UndefOr::from_qv(qv).unwrap()
//         }
//
//         pub(crate ) fn set_bbikkuri(&mut self, bbikkuri : UndefOr<bool>){
//             root::set_bool(self.ptr(), "bbikkuri", bbikkuri.into_qv());
//         }
//
//         pub(crate ) fn sbikkuri(&self) -> UndefOr<String>{
//             let qv = root::get_str(self.ptr(), "sbikkuri").unwrap();
//             UndefOr::from_qv(qv).unwrap()
//         }
//
//         pub(crate ) fn set_sbikkuri(&mut self, sbikkuri : UndefOr<String>){
//             root::set_str(self.ptr(), "sbikkuri", sbikkuri.into_qv());
//         }
//
//         pub(crate ) fn int(&self) -> i64{
//             let qv = root::get_int(self.ptr(), "int").unwrap();
//             qv.into_value().unwrap()
//         }
//
//         pub(crate ) fn set_int(&mut self, int : i64){
//             root::set_int(self.ptr(), "int", Qv::Val(int));
//         }
//
//         pub(crate ) fn sbh(&self) -> Qv<String>{
//             let qv = root::get_str(self.ptr(), "sbh").unwrap();
//             qv
//         }
//
//         pub(crate ) fn set_sbh(&mut self, sbh : Qv<String>){
//             root::set_str(self.ptr(), "sbh", sbh.into_qv());
//         }
//
//         pub(crate ) fn s(&self) -> String{
//             let qv = root::get_str(self.ptr(), "s").unwrap();
//             qv.into_value().unwrap()
//         }
//
//         pub(crate ) fn set_s(&mut self, s : String){
//             root::set_str(self.ptr(), "s", Qv::Val(s));
//         }
//
//         pub(crate ) fn bbh(&self) -> Qv<bool>{
//             let qv = root::get_bool(self.ptr(), "bbh").unwrap();
//             qv
//         }
//
//         pub(crate ) fn set_bbh(&mut self, bbh : Qv<bool>){
//             root::set_bool(self.ptr(), "bbh", bbh.into_qv());
//         }
//
//         pub(crate ) fn shatena(&self) -> NullOr<String>{
//             let qv = root::get_str(self.ptr(), "shatena").unwrap();
//             NullOr::from_qv(qv).unwrap()
//         }
//
//         pub(crate ) fn set_shatena(&mut self, shatena : NullOr<String>){
//             root::set_str(self.ptr(), "shatena", shatena.into_qv());
//         }
//
//         pub(crate ) fn bhatena(&self) -> NullOr<bool>{
//             let qv = root::get_bool(self.ptr(), "bhatena").unwrap();
//             NullOr::from_qv(qv).unwrap()
//         }
//
//         pub(crate ) fn set_bhatena(&mut self, bhatena : NullOr<bool>){
//             root::set_bool(self.ptr(), "bhatena", bhatena.into_qv());
//         }
//
//         pub(crate ) fn b(&self) -> bool{
//             let qv = root::get_bool(self.ptr(), "b").unwrap();
//             qv.into_value().unwrap()
//         }
//
//         pub(crate ) fn set_b(&mut self, b : bool){
//             root::set_bool(self.ptr(), "b", Qv::Val(b));
//         }
//
//         pub(crate ) fn float(&self) -> f64{
//             let qv = root::get_float(self.ptr(), "float").unwrap();
//             qv.into_value().unwrap()
//         }
//
//         pub(crate ) fn set_float(&mut self, float : f64){
//             root::set_float(self.ptr(), "float", Qv::Val(float));
//         }
//
//     }
//
// }
