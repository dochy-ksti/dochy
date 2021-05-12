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

	pub fn data1(&self) -> String{
		let qv = root::get_str(self.ptr, "data1").unwrap();
		qv.into_value().unwrap()
	}
	pub fn data1_def_val(&self) -> String{
		let qv = root::get_str_def(self.ptr, "data1").unwrap();
		qv.into_value().unwrap()
	}
	pub fn data1_immutable(&self) -> &String{
		let qv = root::get_immutable_str(self.ptr, "data1").unwrap();
		qv.into_value().unwrap()
	}
	pub fn data1_mutable(&mut self) -> &mut String{
		let qv = root::get_mutable_str(self.ptr, "data1").unwrap();
		qv.into_value().unwrap()
	}
	pub fn set_data1(&mut self, data1 : String){
		root::set_str(self.ptr, "data1", Qv::Val(data1));
	}
	pub fn data2(&self) -> String{
		let qv = root::get_str(self.ptr, "data2").unwrap();
		qv.into_value().unwrap()
	}
	pub fn data2_def_val(&self) -> String{
		let qv = root::get_str_def(self.ptr, "data2").unwrap();
		qv.into_value().unwrap()
	}
	pub fn data2_immutable(&self) -> &String{
		let qv = root::get_immutable_str(self.ptr, "data2").unwrap();
		qv.into_value().unwrap()
	}
	pub fn data2_mutable(&mut self) -> &mut String{
		let qv = root::get_mutable_str(self.ptr, "data2").unwrap();
		qv.into_value().unwrap()
	}
	pub fn set_data2(&mut self, data2 : String){
		root::set_str(self.ptr, "data2", Qv::Val(data2));
	}
	pub fn data3(&self) -> String{
		let qv = root::get_str(self.ptr, "data3").unwrap();
		qv.into_value().unwrap()
	}
	pub fn data3_def_val(&self) -> String{
		let qv = root::get_str_def(self.ptr, "data3").unwrap();
		qv.into_value().unwrap()
	}
	pub fn data3_immutable(&self) -> &String{
		let qv = root::get_immutable_str(self.ptr, "data3").unwrap();
		qv.into_value().unwrap()
	}
	pub fn data3_mutable(&mut self) -> &mut String{
		let qv = root::get_mutable_str(self.ptr, "data3").unwrap();
		qv.into_value().unwrap()
	}
	pub fn set_data3(&mut self, data3 : String){
		root::set_str(self.ptr, "data3", Qv::Val(data3));
	}
}
