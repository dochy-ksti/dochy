#[cfg(test)] pub mod test{
	use nougami_core::intf::*;
	use nougami_core::structs::*;
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
	
		pub fn refed2(&self) -> Refed2Table{
			let ans = root::get_table(self.ptr, "refed2").unwrap();
			Refed2Table::new(ans)
		}
		pub fn list(&mut self) -> MListPtr<ListMItem>{
			root::get_mlist(self.ptr, "list").unwrap()
		}
		pub fn refed1(&self) -> Refed1Table{
			let ans = root::get_table(self.ptr, "refed1").unwrap();
			Refed1Table::new(ans)
		}
	}
	#[derive(Debug, PartialEq)]
	pub struct Refed2Table {
		ptr : TablePtr,
	}
	impl Refed2Table {
		pub fn new(ptr : TablePtr) -> Refed2Table{ Refed2Table{ ptr } } 
		pub fn b1(&self) -> Refed2CItem {
			let ptr = table::get_value(self.ptr, "b1").unwrap();
			Refed2CItem::from(ptr)
		}
		pub fn b2(&self) -> Refed2CItem {
			let ptr = table::get_value(self.ptr, "b2").unwrap();
			Refed2CItem::from(ptr)
		}
		pub fn from_id(&self, id : Refed2TableID) -> Refed2CItem{
			match id{
				Refed2TableID::B1 => self.b1(),
				Refed2TableID::B2 => self.b2(),
			}
		}
	}
	#[repr(u64)] pub enum Refed2TableID{ B1, B2, }
	impl Refed2TableID{
		pub fn from_str(id : &str) -> Option<Self>{
			match id{
				"b1" => Some(Self::B1),
				"b2" => Some(Self::B2),
				_ =>{ None }
			}
		}
		pub fn from_num(id : usize) -> Self{
			match id{
				0 => Self::B1,
				1 => Self::B2,
				_ => panic!("invalid ID num {} Refed2TableID", id),
			}
		}
		pub fn len() -> usize{ 2 }
		pub fn to_num(&self) -> usize{
			match self{
				Refed2TableID::B1 => 0,
				Refed2TableID::B2 => 1,
			}
		}
		pub fn metadata() -> &'static [&'static str]{
			&["b1", "b2", ]
		}
		pub fn to_str(&self) -> &'static str{
			Self::metadata()[self.to_num()]
		}
	}
	#[derive(Debug, PartialEq)]
	pub struct Refed2CItem {
		ptr : CItemPtr,
	}
	impl From<CItemPtr> for Refed2CItem {
		fn from(ptr : CItemPtr) -> Self { Self{ ptr } }
	}
	impl Refed2CItem {
		pub fn not_very_important(&self) -> i64{
			let qv = citem::get_int(self.ptr, "notVeryImportant").unwrap();
			qv.into_value().unwrap()
		}
		
	}
	
	#[derive(Debug, PartialEq)]
	pub struct ListMItem {
		ptr : MItemPtr,
	}
	impl From<MItemPtr> for ListMItem {
		fn from(ptr : MItemPtr) -> Self {
			Self{ ptr }
		}
	}
	impl ListMItem {
		pub fn mem(&self) -> i64{
			let qv = mitem::get_int(self.ptr, "mem").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem(&mut self, mem : i64){
			mitem::set_int(self.ptr, "mem", Qv::Val(mem));
		}
		pub fn in_list(&mut self) -> MListPtr<InListMItem>{
			mitem::get_mil(self.ptr, "inList").unwrap().unwrap()
		}
		pub fn ref_refed1(&self) -> Refed1CItem{
			let qv = mitem::get_ref(self.ptr, "refed1").unwrap();
			Refed1CItem::from(qv.into_value().unwrap())
		}
		pub fn ref_id_refed1(&self) -> String{
			let qv = mitem::get_ref_id(self.ptr, "refed1").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_ref_refed1(&self, id : Refed1TableID){
			mitem::set_ref(self.ptr, "refed1", Qv::Val(id.to_str().to_string()));
		}
	}
	#[derive(Debug, PartialEq)]
	pub struct InListMItem {
		ptr : MItemPtr,
	}
	impl From<MItemPtr> for InListMItem {
		fn from(ptr : MItemPtr) -> Self {
			Self{ ptr }
		}
	}
	impl InListMItem {
		pub fn in_mem(&self) -> i64{
			let qv = mitem::get_int(self.ptr, "inMem").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_in_mem(&mut self, in_mem : i64){
			mitem::set_int(self.ptr, "inMem", Qv::Val(in_mem));
		}
		pub fn in_list2(&mut self) -> MListPtr<InList2MItem>{
			mitem::get_mil(self.ptr, "inList2").unwrap().unwrap()
		}
		pub fn ref_refed2(&self) -> Refed2CItem{
			let qv = mitem::get_ref(self.ptr, "refed2").unwrap();
			Refed2CItem::from(qv.into_value().unwrap())
		}
		pub fn ref_id_refed2(&self) -> String{
			let qv = mitem::get_ref_id(self.ptr, "refed2").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_ref_refed2(&self, id : Refed2TableID){
			mitem::set_ref(self.ptr, "refed2", Qv::Val(id.to_str().to_string()));
		}
	}
	#[derive(Debug, PartialEq)]
	pub struct InList2MItem {
		ptr : MItemPtr,
	}
	impl From<MItemPtr> for InList2MItem {
		fn from(ptr : MItemPtr) -> Self {
			Self{ ptr }
		}
	}
	impl InList2MItem {
		pub fn in_mem2(&self) -> i64{
			let qv = mitem::get_int(self.ptr, "inMem2").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_in_mem2(&mut self, in_mem2 : i64){
			mitem::set_int(self.ptr, "inMem2", Qv::Val(in_mem2));
		}
		
		
	}
	
	#[derive(Debug, PartialEq)]
	pub struct Refed1Table {
		ptr : TablePtr,
	}
	impl Refed1Table {
		pub fn new(ptr : TablePtr) -> Refed1Table{ Refed1Table{ ptr } } 
		pub fn a1(&self) -> Refed1CItem {
			let ptr = table::get_value(self.ptr, "a1").unwrap();
			Refed1CItem::from(ptr)
		}
		pub fn a2(&self) -> Refed1CItem {
			let ptr = table::get_value(self.ptr, "a2").unwrap();
			Refed1CItem::from(ptr)
		}
		pub fn from_id(&self, id : Refed1TableID) -> Refed1CItem{
			match id{
				Refed1TableID::A1 => self.a1(),
				Refed1TableID::A2 => self.a2(),
			}
		}
	}
	#[repr(u64)] pub enum Refed1TableID{ A1, A2, }
	impl Refed1TableID{
		pub fn from_str(id : &str) -> Option<Self>{
			match id{
				"a1" => Some(Self::A1),
				"a2" => Some(Self::A2),
				_ =>{ None }
			}
		}
		pub fn from_num(id : usize) -> Self{
			match id{
				0 => Self::A1,
				1 => Self::A2,
				_ => panic!("invalid ID num {} Refed1TableID", id),
			}
		}
		pub fn len() -> usize{ 2 }
		pub fn to_num(&self) -> usize{
			match self{
				Refed1TableID::A1 => 0,
				Refed1TableID::A2 => 1,
			}
		}
		pub fn metadata() -> &'static [&'static str]{
			&["a1", "a2", ]
		}
		pub fn to_str(&self) -> &'static str{
			Self::metadata()[self.to_num()]
		}
	}
	#[derive(Debug, PartialEq)]
	pub struct Refed1CItem {
		ptr : CItemPtr,
	}
	impl From<CItemPtr> for Refed1CItem {
		fn from(ptr : CItemPtr) -> Self { Self{ ptr } }
	}
	impl Refed1CItem {
		pub fn not_important(&self) -> i64{
			let qv = citem::get_int(self.ptr, "notImportant").unwrap();
			qv.into_value().unwrap()
		}
		
	}
	
	
}
