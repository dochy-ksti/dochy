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

	pub fn d0_def_val(&self) -> &String{
		let qv = root::get_str_def(self.ptr, "d0").unwrap();
		qv.into_value().unwrap()
	}
	pub fn d0(&self) -> &String{
		let qv = root::get_immutable_str(self.ptr, "d0").unwrap();
		qv.into_value().unwrap()
	}
	pub fn d0_mut(&mut self) -> &mut String{
		let qv = root::get_mutable_str(self.ptr, "d0").unwrap();
		qv.into_value().unwrap()
	}
	pub fn set_d0(&mut self, d0 : String){
		root::set_str(self.ptr, "d0", Qv::Val(d0));
	}
	pub fn d1_def_val(&self) -> &String{
		let qv = root::get_str_def(self.ptr, "d1").unwrap();
		qv.into_value().unwrap()
	}
	pub fn d1(&self) -> &String{
		let qv = root::get_immutable_str(self.ptr, "d1").unwrap();
		qv.into_value().unwrap()
	}
	pub fn d1_mut(&mut self) -> &mut String{
		let qv = root::get_mutable_str(self.ptr, "d1").unwrap();
		qv.into_value().unwrap()
	}
	pub fn set_d1(&mut self, d1 : String){
		root::set_str(self.ptr, "d1", Qv::Val(d1));
	}
	pub fn d2_def_val(&self) -> &String{
		let qv = root::get_str_def(self.ptr, "d2").unwrap();
		qv.into_value().unwrap()
	}
	pub fn d2(&self) -> &String{
		let qv = root::get_immutable_str(self.ptr, "d2").unwrap();
		qv.into_value().unwrap()
	}
	pub fn d2_mut(&mut self) -> &mut String{
		let qv = root::get_mutable_str(self.ptr, "d2").unwrap();
		qv.into_value().unwrap()
	}
	pub fn set_d2(&mut self, d2 : String){
		root::set_str(self.ptr, "d2", Qv::Val(d2));
	}
	pub fn d3_def_val(&self) -> &String{
		let qv = root::get_str_def(self.ptr, "d3").unwrap();
		qv.into_value().unwrap()
	}
	pub fn d3(&self) -> &String{
		let qv = root::get_immutable_str(self.ptr, "d3").unwrap();
		qv.into_value().unwrap()
	}
	pub fn d3_mut(&mut self) -> &mut String{
		let qv = root::get_mutable_str(self.ptr, "d3").unwrap();
		qv.into_value().unwrap()
	}
	pub fn set_d3(&mut self, d3 : String){
		root::set_str(self.ptr, "d3", Qv::Val(d3));
	}
	pub fn d4_def_val(&self) -> &String{
		let qv = root::get_str_def(self.ptr, "d4").unwrap();
		qv.into_value().unwrap()
	}
	pub fn d4(&self) -> &String{
		let qv = root::get_immutable_str(self.ptr, "d4").unwrap();
		qv.into_value().unwrap()
	}
	pub fn d4_mut(&mut self) -> &mut String{
		let qv = root::get_mutable_str(self.ptr, "d4").unwrap();
		qv.into_value().unwrap()
	}
	pub fn set_d4(&mut self, d4 : String){
		root::set_str(self.ptr, "d4", Qv::Val(d4));
	}
}
