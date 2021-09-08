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

	pub fn undef_nullable_int(&self) -> Qv<i64>{
		let qv = root::get_int(self.ptr, "undef_nullable_int").unwrap();
		qv
	}
	pub fn undef_nullable_int_def_val(&self) -> Qv<i64>{
		let qv = root::get_int_def(self.ptr, "undef_nullable_int").unwrap();
		qv
	}
	pub fn set_undef_nullable_int(&mut self, undef_nullable_int : Qv<i64>){
		root::set_int(self.ptr, "undef_nullable_int", undef_nullable_int.into_qv());
	}
	pub fn undefiable_int(&self) -> UndefOr<i64>{
		let qv = root::get_int(self.ptr, "undefiable_int").unwrap();
		UndefOr::from_qv(qv).unwrap()
	}
	pub fn undefiable_int_def_val(&self) -> UndefOr<i64>{
		let qv = root::get_int_def(self.ptr, "undefiable_int").unwrap();
		UndefOr::from_qv(qv).unwrap()
	}
	pub fn set_undefiable_int(&mut self, undefiable_int : UndefOr<i64>){
		root::set_int(self.ptr, "undefiable_int", undefiable_int.into_qv());
	}
}
