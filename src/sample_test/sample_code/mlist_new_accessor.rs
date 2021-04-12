use dochy_core::intf::*;
use dochy_core::structs::*;
unsafe impl Send for RootIntf{}
#[derive(Debug, PartialEq)]
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
    pub unsafe fn root_obj_ref(&self) -> &RootObject{ self.root.as_ref() }
    pub unsafe fn root_obj_ref_mut(&mut self) -> &mut RootObject{ self.root.as_mut() }

	pub fn mlist(&mut self) -> MListPtr<MlistMItem>{
		root::get_mlist(self.ptr, "mlist").unwrap()
	}
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct MlistMItem {
	ptr : MItemPtr,
}
impl From<MItemPtr> for MlistMItem {
	fn from(ptr : MItemPtr) -> Self {
		Self{ ptr }
	}
}
impl MlistMItem {
	pub fn qux(&self) -> i64{
		let qv = mitem::get_int(self.ptr, "qux").unwrap();
		qv.into_value().unwrap()
	}
	pub fn qux_def_val(&self) -> i64{
		let qv = mitem::get_int_def(self.ptr, "qux").unwrap();
		qv.into_value().unwrap()
	}
	pub fn set_qux(&mut self, qux : i64){
		mitem::set_int(self.ptr, "qux", Qv::Val(qux));
	}
	pub fn baz(&self) -> i64{
		let qv = mitem::get_int(self.ptr, "baz").unwrap();
		qv.into_value().unwrap()
	}
	pub fn baz_def_val(&self) -> i64{
		let qv = mitem::get_int_def(self.ptr, "baz").unwrap();
		qv.into_value().unwrap()
	}
	pub fn set_baz(&mut self, baz : i64){
		mitem::set_int(self.ptr, "baz", Qv::Val(baz));
	}
	
	
}

