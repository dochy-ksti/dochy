use dochy::core::intf::*;
use dochy::core::structs::*;
unsafe impl Send for RootIntf {}
unsafe impl Sync for RootIntf {}
#[derive(Debug)]
pub struct RootIntf{
    root : Box<RootObject>,
    ptr : RootObjectPtr,
}
impl RootIntf{
    pub fn new(obj : RootObject) -> RootIntf{
		let mut root = Box::new(obj);
		let ptr = RootObjectPtr::new(root.as_mut());
		RootIntf { root, ptr }
	}
    pub fn root_obj_ref(&self) -> &RootObject{ self.root.as_ref() }
    pub fn root_obj_ref_mut(&mut self) -> &mut RootObject{ self.root.as_mut() }

	pub fn data0_def_val(&self) -> &Vec<u8>{
		let qv = root::get_binary_def(self.ptr, "data0").unwrap();
		qv.into_value().unwrap()
	}
	pub fn data0(&self) -> &Vec<u8>{
		let qv = root::get_immutable_binary(self.ptr, "data0").unwrap();
		qv.into_value().unwrap()
	}
	pub fn data0_mut(&mut self) -> &mut Vec<u8>{
		let qv = root::get_mutable_binary(self.ptr, "data0").unwrap();
		qv.into_value().unwrap()
	}
	pub fn set_data0(&mut self, data0 : Vec<u8>){
		root::set_binary(self.ptr, "data0", Qv::Val(data0));
	}
	pub fn data9_def_val(&self) -> &Vec<u8>{
		let qv = root::get_binary_def(self.ptr, "data9").unwrap();
		qv.into_value().unwrap()
	}
	pub fn data9(&self) -> &Vec<u8>{
		let qv = root::get_immutable_binary(self.ptr, "data9").unwrap();
		qv.into_value().unwrap()
	}
	pub fn data9_mut(&mut self) -> &mut Vec<u8>{
		let qv = root::get_mutable_binary(self.ptr, "data9").unwrap();
		qv.into_value().unwrap()
	}
	pub fn set_data9(&mut self, data9 : Vec<u8>){
		root::set_binary(self.ptr, "data9", Qv::Val(data9));
	}
	pub fn data6_def_val(&self) -> &Vec<u8>{
		let qv = root::get_binary_def(self.ptr, "data6").unwrap();
		qv.into_value().unwrap()
	}
	pub fn data6(&self) -> &Vec<u8>{
		let qv = root::get_immutable_binary(self.ptr, "data6").unwrap();
		qv.into_value().unwrap()
	}
	pub fn data6_mut(&mut self) -> &mut Vec<u8>{
		let qv = root::get_mutable_binary(self.ptr, "data6").unwrap();
		qv.into_value().unwrap()
	}
	pub fn set_data6(&mut self, data6 : Vec<u8>){
		root::set_binary(self.ptr, "data6", Qv::Val(data6));
	}
	pub fn data4_def_val(&self) -> &Vec<u8>{
		let qv = root::get_binary_def(self.ptr, "data4").unwrap();
		qv.into_value().unwrap()
	}
	pub fn data4(&self) -> &Vec<u8>{
		let qv = root::get_immutable_binary(self.ptr, "data4").unwrap();
		qv.into_value().unwrap()
	}
	pub fn data4_mut(&mut self) -> &mut Vec<u8>{
		let qv = root::get_mutable_binary(self.ptr, "data4").unwrap();
		qv.into_value().unwrap()
	}
	pub fn set_data4(&mut self, data4 : Vec<u8>){
		root::set_binary(self.ptr, "data4", Qv::Val(data4));
	}
	pub fn data3_def_val(&self) -> &Vec<u8>{
		let qv = root::get_binary_def(self.ptr, "data3").unwrap();
		qv.into_value().unwrap()
	}
	pub fn data3(&self) -> &Vec<u8>{
		let qv = root::get_immutable_binary(self.ptr, "data3").unwrap();
		qv.into_value().unwrap()
	}
	pub fn data3_mut(&mut self) -> &mut Vec<u8>{
		let qv = root::get_mutable_binary(self.ptr, "data3").unwrap();
		qv.into_value().unwrap()
	}
	pub fn set_data3(&mut self, data3 : Vec<u8>){
		root::set_binary(self.ptr, "data3", Qv::Val(data3));
	}
	pub fn data8_def_val(&self) -> &Vec<u8>{
		let qv = root::get_binary_def(self.ptr, "data8").unwrap();
		qv.into_value().unwrap()
	}
	pub fn data8(&self) -> &Vec<u8>{
		let qv = root::get_immutable_binary(self.ptr, "data8").unwrap();
		qv.into_value().unwrap()
	}
	pub fn data8_mut(&mut self) -> &mut Vec<u8>{
		let qv = root::get_mutable_binary(self.ptr, "data8").unwrap();
		qv.into_value().unwrap()
	}
	pub fn set_data8(&mut self, data8 : Vec<u8>){
		root::set_binary(self.ptr, "data8", Qv::Val(data8));
	}
	pub fn data1_def_val(&self) -> &Vec<u8>{
		let qv = root::get_binary_def(self.ptr, "data1").unwrap();
		qv.into_value().unwrap()
	}
	pub fn data1(&self) -> &Vec<u8>{
		let qv = root::get_immutable_binary(self.ptr, "data1").unwrap();
		qv.into_value().unwrap()
	}
	pub fn data1_mut(&mut self) -> &mut Vec<u8>{
		let qv = root::get_mutable_binary(self.ptr, "data1").unwrap();
		qv.into_value().unwrap()
	}
	pub fn set_data1(&mut self, data1 : Vec<u8>){
		root::set_binary(self.ptr, "data1", Qv::Val(data1));
	}
	pub fn data7_def_val(&self) -> &Vec<u8>{
		let qv = root::get_binary_def(self.ptr, "data7").unwrap();
		qv.into_value().unwrap()
	}
	pub fn data7(&self) -> &Vec<u8>{
		let qv = root::get_immutable_binary(self.ptr, "data7").unwrap();
		qv.into_value().unwrap()
	}
	pub fn data7_mut(&mut self) -> &mut Vec<u8>{
		let qv = root::get_mutable_binary(self.ptr, "data7").unwrap();
		qv.into_value().unwrap()
	}
	pub fn set_data7(&mut self, data7 : Vec<u8>){
		root::set_binary(self.ptr, "data7", Qv::Val(data7));
	}
	pub fn data5_def_val(&self) -> &Vec<u8>{
		let qv = root::get_binary_def(self.ptr, "data5").unwrap();
		qv.into_value().unwrap()
	}
	pub fn data5(&self) -> &Vec<u8>{
		let qv = root::get_immutable_binary(self.ptr, "data5").unwrap();
		qv.into_value().unwrap()
	}
	pub fn data5_mut(&mut self) -> &mut Vec<u8>{
		let qv = root::get_mutable_binary(self.ptr, "data5").unwrap();
		qv.into_value().unwrap()
	}
	pub fn set_data5(&mut self, data5 : Vec<u8>){
		root::set_binary(self.ptr, "data5", Qv::Val(data5));
	}
	pub fn data2_def_val(&self) -> &Vec<u8>{
		let qv = root::get_binary_def(self.ptr, "data2").unwrap();
		qv.into_value().unwrap()
	}
	pub fn data2(&self) -> &Vec<u8>{
		let qv = root::get_immutable_binary(self.ptr, "data2").unwrap();
		qv.into_value().unwrap()
	}
	pub fn data2_mut(&mut self) -> &mut Vec<u8>{
		let qv = root::get_mutable_binary(self.ptr, "data2").unwrap();
		qv.into_value().unwrap()
	}
	pub fn set_data2(&mut self, data2 : Vec<u8>){
		root::set_binary(self.ptr, "data2", Qv::Val(data2));
	}
}
