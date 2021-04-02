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

	pub fn table(&self) -> TableTable{
		let ans = root::get_table(self.ptr, "table").unwrap();
		TableTable::new(ans)
	}
	pub fn list(&mut self) -> MListPtr<ListMItem>{
		root::get_mlist(self.ptr, "list").unwrap()
	}
}
#[derive(Debug, PartialEq)]
pub struct TableTable {
	ptr : TablePtr,
}
impl TableTable {
	pub fn new(ptr : TablePtr) -> TableTable{ TableTable{ ptr } } 
	pub fn item2(&self) -> TableCItem {
		let ptr = table::get_value(self.ptr, "item2").unwrap();
		TableCItem::from(ptr)
	}
	pub fn item1(&self) -> TableCItem {
		let ptr = table::get_value(self.ptr, "item1").unwrap();
		TableCItem::from(ptr)
	}
	pub fn get_by_id(&self, id : TableTableID) -> TableCItem{
		match id{
			TableTableID::Item2 => self.item2(),
			TableTableID::Item1 => self.item1(),
		}
	}
}
#[repr(u64)] pub enum TableTableID{ Item2, Item1, }
impl TableTableID{
	pub fn from_str(id : &str) -> Option<Self>{
		match id{
			"item2" => Some(Self::Item2),
			"item1" => Some(Self::Item1),
			_ =>{ None }
		}
	}
	pub fn from_num(id : usize) -> Self{
		match id{
			0 => Self::Item2,
			1 => Self::Item1,
			_ => panic!("invalid ID num {} TableTableID", id),
		}
	}
	pub fn len() -> usize{ 2 }
	pub fn to_num(&self) -> usize{
		match self{
			TableTableID::Item2 => 0,
			TableTableID::Item1 => 1,
		}
	}
	pub fn metadata() -> &'static [&'static str]{
		&["item2", "item1", ]
	}
	pub fn to_str(&self) -> &'static str{
		Self::metadata()[self.to_num()]
	}
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct TableCItem {
	ptr : CItemPtr,
}
impl From<CItemPtr> for TableCItem {
	fn from(ptr : CItemPtr) -> Self { Self{ ptr } }
}
impl TableCItem {
	pub fn foo(&self) -> i64{
		let qv = citem::get_int(self.ptr, "foo").unwrap();
		qv.into_value().unwrap()
	}
	pub fn foo_def_val(&self) -> i64{
		let qv = citem::get_int_def(self.ptr, "foo").unwrap();
		qv.into_value().unwrap()
	}
	
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ListMItem {
	ptr : MItemPtr,
}
impl From<MItemPtr> for ListMItem {
	fn from(ptr : MItemPtr) -> Self {
		Self{ ptr }
	}
}
impl ListMItem {
	pub fn bar(&self) -> i64{
		let qv = mitem::get_int(self.ptr, "bar").unwrap();
		qv.into_value().unwrap()
	}
	pub fn bar_def_val(&self) -> i64{
		let qv = mitem::get_int_def(self.ptr, "bar").unwrap();
		qv.into_value().unwrap()
	}
	pub fn set_bar(&mut self, bar : i64){
		mitem::set_int(self.ptr, "bar", Qv::Val(bar));
	}
	pub fn ref_table(&self) -> TableCItem{
		let qv = mitem::get_ref(self.ptr, "table").unwrap();
		TableCItem::from(qv.into_value().unwrap())
	}
	pub fn ref_id_table(&self) -> String{
		let qv = mitem::get_ref_id(self.ptr, "table").unwrap();
		qv.into_value().unwrap()
	}
	pub fn set_ref_table(&self, id : TableTableID){
		mitem::set_ref(self.ptr, "table", Qv::Val(id.to_str().to_string()));
	}
}

