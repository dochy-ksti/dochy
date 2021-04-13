use dochy::core::intf::*;
use dochy::core::structs::*;
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

	pub fn message(&self) -> String{
		let qv = root::get_str(self.ptr, "message").unwrap();
		qv.into_value().unwrap()
	}
	pub fn message_def_val(&self) -> String{
		let qv = root::get_str_def(self.ptr, "message").unwrap();
		qv.into_value().unwrap()
	}
	pub fn set_message(&mut self, message : String){
		root::set_str(self.ptr, "message", Qv::Val(message));
	}
}
