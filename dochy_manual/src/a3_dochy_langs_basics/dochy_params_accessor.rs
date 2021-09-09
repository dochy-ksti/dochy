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

	pub fn float_value(&self) -> f64{
		let qv = root::get_float(self.ptr, "float_value").unwrap();
		qv.into_value().unwrap()
	}
	pub fn float_value_def_val(&self) -> f64{
		let qv = root::get_float_def(self.ptr, "float_value").unwrap();
		qv.into_value().unwrap()
	}
	pub fn set_float_value(&mut self, float_value : f64){
		root::set_float(self.ptr, "float_value", Qv::Val(float_value));
	}
	pub fn bool_value(&self) -> bool{
		let qv = root::get_bool(self.ptr, "bool_value").unwrap();
		qv.into_value().unwrap()
	}
	pub fn bool_value_def_val(&self) -> bool{
		let qv = root::get_bool_def(self.ptr, "bool_value").unwrap();
		qv.into_value().unwrap()
	}
	pub fn set_bool_value(&mut self, bool_value : bool){
		root::set_bool(self.ptr, "bool_value", Qv::Val(bool_value));
	}
	pub fn float_array_def_val(&self) -> &Vec<f64>{
		let qv = root::get_float_array_def(self.ptr, "float_array").unwrap();
		qv.into_value().unwrap()
	}
	pub fn float_array(&self) -> &Vec<f64>{
		let qv = root::get_immutable_float_array(self.ptr, "float_array").unwrap();
		qv.into_value().unwrap()
	}
	pub fn float_array_mut(&mut self) -> &mut Vec<f64>{
		let qv = root::get_mutable_float_array(self.ptr, "float_array").unwrap();
		qv.into_value().unwrap()
	}
	pub fn set_float_array(&mut self, float_array : Vec<f64>){
		root::set_float_array(self.ptr, "float_array", Qv::Val(float_array));
	}
	pub fn binary_def_val(&self) -> &Vec<u8>{
		let qv = root::get_binary_def(self.ptr, "binary").unwrap();
		qv.into_value().unwrap()
	}
	pub fn binary(&self) -> &Vec<u8>{
		let qv = root::get_immutable_binary(self.ptr, "binary").unwrap();
		qv.into_value().unwrap()
	}
	pub fn binary_mut(&mut self) -> &mut Vec<u8>{
		let qv = root::get_mutable_binary(self.ptr, "binary").unwrap();
		qv.into_value().unwrap()
	}
	pub fn set_binary(&mut self, binary : Vec<u8>){
		root::set_binary(self.ptr, "binary", Qv::Val(binary));
	}
	pub fn int_value(&self) -> i64{
		let qv = root::get_int(self.ptr, "int_value").unwrap();
		qv.into_value().unwrap()
	}
	pub fn int_value_def_val(&self) -> i64{
		let qv = root::get_int_def(self.ptr, "int_value").unwrap();
		qv.into_value().unwrap()
	}
	pub fn set_int_value(&mut self, int_value : i64){
		root::set_int(self.ptr, "int_value", Qv::Val(int_value));
	}
	pub fn int_array_def_val(&self) -> &Vec<i64>{
		let qv = root::get_int_array_def(self.ptr, "int_array").unwrap();
		qv.into_value().unwrap()
	}
	pub fn int_array(&self) -> &Vec<i64>{
		let qv = root::get_immutable_int_array(self.ptr, "int_array").unwrap();
		qv.into_value().unwrap()
	}
	pub fn int_array_mut(&mut self) -> &mut Vec<i64>{
		let qv = root::get_mutable_int_array(self.ptr, "int_array").unwrap();
		qv.into_value().unwrap()
	}
	pub fn set_int_array(&mut self, int_array : Vec<i64>){
		root::set_int_array(self.ptr, "int_array", Qv::Val(int_array));
	}
	pub fn string_value_def_val(&self) -> &String{
		let qv = root::get_str_def(self.ptr, "string_value").unwrap();
		qv.into_value().unwrap()
	}
	pub fn string_value(&self) -> &String{
		let qv = root::get_immutable_str(self.ptr, "string_value").unwrap();
		qv.into_value().unwrap()
	}
	pub fn string_value_mut(&mut self) -> &mut String{
		let qv = root::get_mutable_str(self.ptr, "string_value").unwrap();
		qv.into_value().unwrap()
	}
	pub fn set_string_value(&mut self, string_value : String){
		root::set_str(self.ptr, "string_value", Qv::Val(string_value));
	}
}
