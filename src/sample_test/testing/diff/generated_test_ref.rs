#[cfg(test)] pub mod test{
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
	
		pub fn list1(&mut self) -> MListPtr<List1MItem>{
			root::get_mlist(self.ptr, "list1").unwrap()
		}
		pub fn list2(&mut self) -> MListPtr<List2MItem>{
			root::get_mlist(self.ptr, "list2").unwrap()
		}
		pub fn list3(&mut self) -> MListPtr<List3MItem>{
			root::get_mlist(self.ptr, "list3").unwrap()
		}
		pub fn refed1(&self) -> Refed1Table{
			let ans = root::get_table(self.ptr, "refed1").unwrap();
			Refed1Table::new(ans)
		}
		pub fn refed2(&self) -> Refed2Table{
			let ans = root::get_table(self.ptr, "refed2").unwrap();
			Refed2Table::new(ans)
		}
		pub fn refed3(&self) -> Refed3Table{
			let ans = root::get_table(self.ptr, "refed3").unwrap();
			Refed3Table::new(ans)
		}
		pub fn refed4(&self) -> Refed4Table{
			let ans = root::get_table(self.ptr, "refed4").unwrap();
			Refed4Table::new(ans)
		}
	}
	#[derive(Debug, PartialEq)]
	pub struct List1MItem {
		ptr : MItemPtr,
	}
	impl From<MItemPtr> for List1MItem {
		fn from(ptr : MItemPtr) -> Self {
			Self{ ptr }
		}
	}
	impl List1MItem {
		pub fn ref_refed1(&self) -> Refed1CItem{
			let qv = mitem::get_ref(self.ptr, "refed1").unwrap();
			Refed1CItem::from(qv.into_value().unwrap())
		}
		pub fn ref_id_refed1(&self) -> String{
			let qv = mitem::get_ref_id(self.ptr, "refed1").unwrap();
			qv.into_value().unwrap()
		}
		pub fn ref_refed2(&self) -> NullOr<Refed2CItem>{
			let qv = mitem::get_ref(self.ptr, "refed2").unwrap();
			NullOr::from_qv(qv).unwrap().map(|p| Refed2CItem::from(*p))
		}
		pub fn ref_id_refed2(&self) -> NullOr<String>{
			let qv = mitem::get_ref_id(self.ptr, "refed2").unwrap();
			NullOr::from_qv(qv).unwrap()
		}
		pub fn ref_refed3(&self) -> UndefOr<Refed3CItem>{
			let qv = mitem::get_ref(self.ptr, "refed3").unwrap();
			UndefOr::from_qv(qv).unwrap().map(|p| Refed3CItem::from(*p))
		}
		pub fn ref_id_refed3(&self) -> UndefOr<String>{
			let qv = mitem::get_ref_id(self.ptr, "refed3").unwrap();
			UndefOr::from_qv(qv).unwrap()
		}
		pub fn ref_refed4(&self) -> Qv<Refed4CItem>{
			let qv = mitem::get_ref(self.ptr, "refed4").unwrap();
			qv.map(|p| Refed4CItem::from(*p))
		}
		pub fn ref_id_refed4(&self) -> Qv<String>{
			let qv = mitem::get_ref_id(self.ptr, "refed4").unwrap();
			qv
		}
		pub fn set_ref_refed1(&self, id : Refed1TableID){
			mitem::set_ref(self.ptr, "refed1", Qv::Val(id.to_str().to_string()));
		}
		pub fn set_ref_refed2(&self, id : NullOr<Refed2TableID>){
			mitem::set_ref(self.ptr, "refed2", id.into_qv().map(|v| v.to_str().to_string()));
		}
		pub fn set_ref_refed3(&self, id : UndefOr<Refed3TableID>){
			mitem::set_ref(self.ptr, "refed3", id.into_qv().map(|v| v.to_str().to_string()));
		}
		pub fn set_ref_refed4(&self, id : Qv<Refed4TableID>){
			mitem::set_ref(self.ptr, "refed4", id.into_qv().map(|v| v.to_str().to_string()));
		}
	}
	
	#[derive(Debug, PartialEq)]
	pub struct List2MItem {
		ptr : MItemPtr,
	}
	impl From<MItemPtr> for List2MItem {
		fn from(ptr : MItemPtr) -> Self {
			Self{ ptr }
		}
	}
	impl List2MItem {
		pub fn ref_refed1(&self) -> Refed1CItem{
			let qv = mitem::get_ref(self.ptr, "refed1").unwrap();
			Refed1CItem::from(qv.into_value().unwrap())
		}
		pub fn ref_id_refed1(&self) -> String{
			let qv = mitem::get_ref_id(self.ptr, "refed1").unwrap();
			qv.into_value().unwrap()
		}
		pub fn ref_refed2(&self) -> NullOr<Refed2CItem>{
			let qv = mitem::get_ref(self.ptr, "refed2").unwrap();
			NullOr::from_qv(qv).unwrap().map(|p| Refed2CItem::from(*p))
		}
		pub fn ref_id_refed2(&self) -> NullOr<String>{
			let qv = mitem::get_ref_id(self.ptr, "refed2").unwrap();
			NullOr::from_qv(qv).unwrap()
		}
		pub fn ref_refed3(&self) -> UndefOr<Refed3CItem>{
			let qv = mitem::get_ref(self.ptr, "refed3").unwrap();
			UndefOr::from_qv(qv).unwrap().map(|p| Refed3CItem::from(*p))
		}
		pub fn ref_id_refed3(&self) -> UndefOr<String>{
			let qv = mitem::get_ref_id(self.ptr, "refed3").unwrap();
			UndefOr::from_qv(qv).unwrap()
		}
		pub fn ref_refed4(&self) -> Qv<Refed4CItem>{
			let qv = mitem::get_ref(self.ptr, "refed4").unwrap();
			qv.map(|p| Refed4CItem::from(*p))
		}
		pub fn ref_id_refed4(&self) -> Qv<String>{
			let qv = mitem::get_ref_id(self.ptr, "refed4").unwrap();
			qv
		}
		pub fn set_ref_refed1(&self, id : Refed1TableID){
			mitem::set_ref(self.ptr, "refed1", Qv::Val(id.to_str().to_string()));
		}
		pub fn set_ref_refed2(&self, id : NullOr<Refed2TableID>){
			mitem::set_ref(self.ptr, "refed2", id.into_qv().map(|v| v.to_str().to_string()));
		}
		pub fn set_ref_refed3(&self, id : UndefOr<Refed3TableID>){
			mitem::set_ref(self.ptr, "refed3", id.into_qv().map(|v| v.to_str().to_string()));
		}
		pub fn set_ref_refed4(&self, id : Qv<Refed4TableID>){
			mitem::set_ref(self.ptr, "refed4", id.into_qv().map(|v| v.to_str().to_string()));
		}
	}
	
	#[derive(Debug, PartialEq)]
	pub struct List3MItem {
		ptr : MItemPtr,
	}
	impl From<MItemPtr> for List3MItem {
		fn from(ptr : MItemPtr) -> Self {
			Self{ ptr }
		}
	}
	impl List3MItem {
		pub fn ref_refed1(&self) -> Refed1CItem{
			let qv = mitem::get_ref(self.ptr, "refed1").unwrap();
			Refed1CItem::from(qv.into_value().unwrap())
		}
		pub fn ref_id_refed1(&self) -> String{
			let qv = mitem::get_ref_id(self.ptr, "refed1").unwrap();
			qv.into_value().unwrap()
		}
		pub fn ref_refed2(&self) -> NullOr<Refed2CItem>{
			let qv = mitem::get_ref(self.ptr, "refed2").unwrap();
			NullOr::from_qv(qv).unwrap().map(|p| Refed2CItem::from(*p))
		}
		pub fn ref_id_refed2(&self) -> NullOr<String>{
			let qv = mitem::get_ref_id(self.ptr, "refed2").unwrap();
			NullOr::from_qv(qv).unwrap()
		}
		pub fn ref_refed3(&self) -> UndefOr<Refed3CItem>{
			let qv = mitem::get_ref(self.ptr, "refed3").unwrap();
			UndefOr::from_qv(qv).unwrap().map(|p| Refed3CItem::from(*p))
		}
		pub fn ref_id_refed3(&self) -> UndefOr<String>{
			let qv = mitem::get_ref_id(self.ptr, "refed3").unwrap();
			UndefOr::from_qv(qv).unwrap()
		}
		pub fn ref_refed4(&self) -> Qv<Refed4CItem>{
			let qv = mitem::get_ref(self.ptr, "refed4").unwrap();
			qv.map(|p| Refed4CItem::from(*p))
		}
		pub fn ref_id_refed4(&self) -> Qv<String>{
			let qv = mitem::get_ref_id(self.ptr, "refed4").unwrap();
			qv
		}
		pub fn set_ref_refed1(&self, id : Refed1TableID){
			mitem::set_ref(self.ptr, "refed1", Qv::Val(id.to_str().to_string()));
		}
		pub fn set_ref_refed2(&self, id : NullOr<Refed2TableID>){
			mitem::set_ref(self.ptr, "refed2", id.into_qv().map(|v| v.to_str().to_string()));
		}
		pub fn set_ref_refed3(&self, id : UndefOr<Refed3TableID>){
			mitem::set_ref(self.ptr, "refed3", id.into_qv().map(|v| v.to_str().to_string()));
		}
		pub fn set_ref_refed4(&self, id : Qv<Refed4TableID>){
			mitem::set_ref(self.ptr, "refed4", id.into_qv().map(|v| v.to_str().to_string()));
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
	pub struct Refed3Table {
		ptr : TablePtr,
	}
	impl Refed3Table {
		pub fn new(ptr : TablePtr) -> Refed3Table{ Refed3Table{ ptr } } 
		pub fn c2(&self) -> Refed3CItem {
			let ptr = table::get_value(self.ptr, "c2").unwrap();
			Refed3CItem::from(ptr)
		}
		pub fn c1(&self) -> Refed3CItem {
			let ptr = table::get_value(self.ptr, "c1").unwrap();
			Refed3CItem::from(ptr)
		}
		pub fn from_id(&self, id : Refed3TableID) -> Refed3CItem{
			match id{
				Refed3TableID::C2 => self.c2(),
				Refed3TableID::C1 => self.c1(),
			}
		}
	}
	#[repr(u64)] pub enum Refed3TableID{ C2, C1, }
	impl Refed3TableID{
		pub fn from_str(id : &str) -> Option<Self>{
			match id{
				"c2" => Some(Self::C2),
				"c1" => Some(Self::C1),
				_ =>{ None }
			}
		}
		pub fn from_num(id : usize) -> Self{
			match id{
				0 => Self::C2,
				1 => Self::C1,
				_ => panic!("invalid ID num {} Refed3TableID", id),
			}
		}
		pub fn len() -> usize{ 2 }
		pub fn to_num(&self) -> usize{
			match self{
				Refed3TableID::C2 => 0,
				Refed3TableID::C1 => 1,
			}
		}
		pub fn metadata() -> &'static [&'static str]{
			&["c2", "c1", ]
		}
		pub fn to_str(&self) -> &'static str{
			Self::metadata()[self.to_num()]
		}
	}
	#[derive(Debug, PartialEq)]
	pub struct Refed3CItem {
		ptr : CItemPtr,
	}
	impl From<CItemPtr> for Refed3CItem {
		fn from(ptr : CItemPtr) -> Self { Self{ ptr } }
	}
	impl Refed3CItem {
		pub fn not_so_important(&self) -> i64{
			let qv = citem::get_int(self.ptr, "notSoImportant").unwrap();
			qv.into_value().unwrap()
		}
		
	}
	
	#[derive(Debug, PartialEq)]
	pub struct Refed4Table {
		ptr : TablePtr,
	}
	impl Refed4Table {
		pub fn new(ptr : TablePtr) -> Refed4Table{ Refed4Table{ ptr } } 
		pub fn d2(&self) -> Refed4CItem {
			let ptr = table::get_value(self.ptr, "d2").unwrap();
			Refed4CItem::from(ptr)
		}
		pub fn d1(&self) -> Refed4CItem {
			let ptr = table::get_value(self.ptr, "d1").unwrap();
			Refed4CItem::from(ptr)
		}
		pub fn from_id(&self, id : Refed4TableID) -> Refed4CItem{
			match id{
				Refed4TableID::D2 => self.d2(),
				Refed4TableID::D1 => self.d1(),
			}
		}
	}
	#[repr(u64)] pub enum Refed4TableID{ D2, D1, }
	impl Refed4TableID{
		pub fn from_str(id : &str) -> Option<Self>{
			match id{
				"d2" => Some(Self::D2),
				"d1" => Some(Self::D1),
				_ =>{ None }
			}
		}
		pub fn from_num(id : usize) -> Self{
			match id{
				0 => Self::D2,
				1 => Self::D1,
				_ => panic!("invalid ID num {} Refed4TableID", id),
			}
		}
		pub fn len() -> usize{ 2 }
		pub fn to_num(&self) -> usize{
			match self{
				Refed4TableID::D2 => 0,
				Refed4TableID::D1 => 1,
			}
		}
		pub fn metadata() -> &'static [&'static str]{
			&["d2", "d1", ]
		}
		pub fn to_str(&self) -> &'static str{
			Self::metadata()[self.to_num()]
		}
	}
	#[derive(Debug, PartialEq)]
	pub struct Refed4CItem {
		ptr : CItemPtr,
	}
	impl From<CItemPtr> for Refed4CItem {
		fn from(ptr : CItemPtr) -> Self { Self{ ptr } }
	}
	impl Refed4CItem {
		pub fn not_important(&self) -> i64{
			let qv = citem::get_int(self.ptr, "notImportant").unwrap();
			qv.into_value().unwrap()
		}
		
	}
	
	
}
