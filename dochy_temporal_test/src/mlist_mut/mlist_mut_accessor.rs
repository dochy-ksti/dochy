use dochy::core::intf::*;
use dochy::core::structs::*;
unsafe impl Send for RootIntf {}
unsafe impl Sync for RootIntf {}
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
    pub fn root_obj_ref(&self) -> &RootObject{ self.root.as_ref() }
    pub fn root_obj_ref_mut(&mut self) -> &mut RootObject{ self.root.as_mut() }

	pub unsafe fn mlist_us(&self) -> MListPtr<MlistMItem>{
		root::get_mlist(self.ptr, "mlist").unwrap()
	}
	pub fn mlist(&self) -> MListConst<MlistMItem>{
		MListConst::new(unsafe{ self.mlist_us() }, self)
	}
	pub fn mlist_mut(&mut self) -> MListMut<MlistMItem>{
		MListMut::new(unsafe{ self.mlist_us() }, self)
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
	pub unsafe fn in_list_us(&self) -> MListPtr<InListMItem>{
		mitem_ptr::get_mil(self.ptr, "inList").unwrap().unwrap()
	}
	pub fn in_list(&self) -> MListConst<InListMItem>{
		MListConst::new(unsafe{ self.in_list_us() }, self)
	}
	pub fn in_list_mut(&mut self) -> MListMut<InListMItem>{
		MListMut::new(unsafe{ self.in_list_us() }, self)
	}
	pub fn bar(&self) -> i64{
		let qv = mitem_ptr::get_int(self.ptr, "bar").unwrap();
		qv.into_value().unwrap()
	}
	pub fn bar_def_val(&self) -> i64{
		let qv = mitem_ptr::get_int_def(self.ptr, "bar").unwrap();
		qv.into_value().unwrap()
	}
	pub fn set_bar(&mut self, bar : i64){
		mitem_ptr::set_int(self.ptr, "bar", Qv::Val(bar));
	}
	pub fn baz(&self) -> i64{
		let qv = mitem_ptr::get_int(self.ptr, "baz").unwrap();
		qv.into_value().unwrap()
	}
	pub fn baz_def_val(&self) -> i64{
		let qv = mitem_ptr::get_int_def(self.ptr, "baz").unwrap();
		qv.into_value().unwrap()
	}
	pub fn set_baz(&mut self, baz : i64){
		mitem_ptr::set_int(self.ptr, "baz", Qv::Val(baz));
	}
	
	
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct InListMItem {
	ptr : MItemPtr,
}
impl From<MItemPtr> for InListMItem {
	fn from(ptr : MItemPtr) -> Self {
		Self{ ptr }
	}
}
impl InListMItem {
	pub fn a(&self) -> i64{
		let qv = mitem_ptr::get_int(self.ptr, "a").unwrap();
		qv.into_value().unwrap()
	}
	pub fn a_def_val(&self) -> i64{
		let qv = mitem_ptr::get_int_def(self.ptr, "a").unwrap();
		qv.into_value().unwrap()
	}
	pub fn set_a(&mut self, a : i64){
		mitem_ptr::set_int(self.ptr, "a", Qv::Val(a));
	}
	
	
}

