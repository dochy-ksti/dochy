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

	pub fn nullable_str_def_val(&self) -> NullOr<&String>{
		let qv = root::get_str_def(self.ptr, "nullable_str").unwrap();
		NullOr::from_qv(qv).unwrap()
	}
	pub fn nullable_str(&self) -> NullOr<&String>{
		let qv = root::get_immutable_str(self.ptr, "nullable_str").unwrap();
		NullOr::from_qv(qv).unwrap()
	}
	pub fn nullable_str_mut(&mut self) -> NullOr<&mut String>{
		let qv = root::get_mutable_str(self.ptr, "nullable_str").unwrap();
		NullOr::from_qv(qv).unwrap()
	}
	pub fn set_nullable_str(&mut self, nullable_str : NullOr<String>){
		root::set_str(self.ptr, "nullable_str", nullable_str.into_qv());
	}
	pub fn nullable_float(&self) -> NullOr<f64>{
		let qv = root::get_float(self.ptr, "nullable_float").unwrap();
		NullOr::from_qv(qv).unwrap()
	}
	pub fn nullable_float_def_val(&self) -> NullOr<f64>{
		let qv = root::get_float_def(self.ptr, "nullable_float").unwrap();
		NullOr::from_qv(qv).unwrap()
	}
	pub fn set_nullable_float(&mut self, nullable_float : NullOr<f64>){
		root::set_float(self.ptr, "nullable_float", nullable_float.into_qv());
	}
	pub fn nullable_bool(&self) -> NullOr<bool>{
		let qv = root::get_bool(self.ptr, "nullable_bool").unwrap();
		NullOr::from_qv(qv).unwrap()
	}
	pub fn nullable_bool_def_val(&self) -> NullOr<bool>{
		let qv = root::get_bool_def(self.ptr, "nullable_bool").unwrap();
		NullOr::from_qv(qv).unwrap()
	}
	pub fn set_nullable_bool(&mut self, nullable_bool : NullOr<bool>){
		root::set_bool(self.ptr, "nullable_bool", nullable_bool.into_qv());
	}
	pub fn nullable_int_array_def_val(&self) -> NullOr<&Vec<i64>>{
		let qv = root::get_int_array_def(self.ptr, "nullable_int_array").unwrap();
		NullOr::from_qv(qv).unwrap()
	}
	pub fn nullable_int_array(&self) -> NullOr<&Vec<i64>>{
		let qv = root::get_immutable_int_array(self.ptr, "nullable_int_array").unwrap();
		NullOr::from_qv(qv).unwrap()
	}
	pub fn nullable_int_array_mut(&mut self) -> NullOr<&mut Vec<i64>>{
		let qv = root::get_mutable_int_array(self.ptr, "nullable_int_array").unwrap();
		NullOr::from_qv(qv).unwrap()
	}
	pub fn set_nullable_int_array(&mut self, nullable_int_array : NullOr<Vec<i64>>){
		root::set_int_array(self.ptr, "nullable_int_array", nullable_int_array.into_qv());
	}
	pub fn nullable_float_array_def_val(&self) -> NullOr<&Vec<f64>>{
		let qv = root::get_float_array_def(self.ptr, "nullable_float_array").unwrap();
		NullOr::from_qv(qv).unwrap()
	}
	pub fn nullable_float_array(&self) -> NullOr<&Vec<f64>>{
		let qv = root::get_immutable_float_array(self.ptr, "nullable_float_array").unwrap();
		NullOr::from_qv(qv).unwrap()
	}
	pub fn nullable_float_array_mut(&mut self) -> NullOr<&mut Vec<f64>>{
		let qv = root::get_mutable_float_array(self.ptr, "nullable_float_array").unwrap();
		NullOr::from_qv(qv).unwrap()
	}
	pub fn set_nullable_float_array(&mut self, nullable_float_array : NullOr<Vec<f64>>){
		root::set_float_array(self.ptr, "nullable_float_array", nullable_float_array.into_qv());
	}
	pub fn nullable_binary_def_val(&self) -> NullOr<&Vec<u8>>{
		let qv = root::get_binary_def(self.ptr, "nullable_binary").unwrap();
		NullOr::from_qv(qv).unwrap()
	}
	pub fn nullable_binary(&self) -> NullOr<&Vec<u8>>{
		let qv = root::get_immutable_binary(self.ptr, "nullable_binary").unwrap();
		NullOr::from_qv(qv).unwrap()
	}
	pub fn nullable_binary_mut(&mut self) -> NullOr<&mut Vec<u8>>{
		let qv = root::get_mutable_binary(self.ptr, "nullable_binary").unwrap();
		NullOr::from_qv(qv).unwrap()
	}
	pub fn set_nullable_binary(&mut self, nullable_binary : NullOr<Vec<u8>>){
		root::set_binary(self.ptr, "nullable_binary", nullable_binary.into_qv());
	}
	pub fn nullable_int(&self) -> NullOr<i64>{
		let qv = root::get_int(self.ptr, "nullable_int").unwrap();
		NullOr::from_qv(qv).unwrap()
	}
	pub fn nullable_int_def_val(&self) -> NullOr<i64>{
		let qv = root::get_int_def(self.ptr, "nullable_int").unwrap();
		NullOr::from_qv(qv).unwrap()
	}
	pub fn set_nullable_int(&mut self, nullable_int : NullOr<i64>){
		root::set_int(self.ptr, "nullable_int", nullable_int.into_qv());
	}
}
