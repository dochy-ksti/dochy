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

	pub fn sword(&self) -> SwordTable{
		let ans = root::get_table(self.ptr, "sword").unwrap();
		SwordTable::new(ans)
	}
	pub fn pc_list(&mut self) -> MListPtr<PcListMItem>{
		root::get_mlist(self.ptr, "pcList").unwrap()
	}
	pub fn herb(&self) -> HerbTable{
		let ans = root::get_table(self.ptr, "herb").unwrap();
		HerbTable::new(ans)
	}
}
#[derive(Debug, PartialEq)]
pub struct SwordTable {
	ptr : TablePtr,
}
impl SwordTable {
	pub fn new(ptr : TablePtr) -> SwordTable{ SwordTable{ ptr } } 
	pub fn iron(&self) -> SwordCItem {
		let ptr = table::get_value(self.ptr, "iron").unwrap();
		SwordCItem::from(ptr)
	}
	pub fn bronze(&self) -> SwordCItem {
		let ptr = table::get_value(self.ptr, "bronze").unwrap();
		SwordCItem::from(ptr)
	}
	pub fn get_by_id(&self, id : SwordTableID) -> SwordCItem{
		match id{
			SwordTableID::Iron => self.iron(),
			SwordTableID::Bronze => self.bronze(),
		}
	}
}
#[repr(u64)] pub enum SwordTableID{ Iron, Bronze, }
impl SwordTableID{
	pub fn from_str(id : &str) -> Option<Self>{
		match id{
			"iron" => Some(Self::Iron),
			"bronze" => Some(Self::Bronze),
			_ =>{ None }
		}
	}
	pub fn from_num(id : usize) -> Self{
		match id{
			0 => Self::Iron,
			1 => Self::Bronze,
			_ => panic!("invalid ID num {} SwordTableID", id),
		}
	}
	pub fn len() -> usize{ 2 }
	pub fn to_num(&self) -> usize{
		match self{
			SwordTableID::Iron => 0,
			SwordTableID::Bronze => 1,
		}
	}
	pub fn metadata() -> &'static [&'static str]{
		&["iron", "bronze", ]
	}
	pub fn to_str(&self) -> &'static str{
		Self::metadata()[self.to_num()]
	}
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct SwordCItem {
	ptr : CItemPtr,
}
impl From<CItemPtr> for SwordCItem {
	fn from(ptr : CItemPtr) -> Self { Self{ ptr } }
}
impl SwordCItem {
	pub fn attack(&self) -> i64{
		let qv = citem::get_int(self.ptr, "attack").unwrap();
		qv.into_value().unwrap()
	}
	pub fn attack_def_val(&self) -> i64{
		let qv = citem::get_int_def(self.ptr, "attack").unwrap();
		qv.into_value().unwrap()
	}
	pub fn price(&self) -> i64{
		let qv = citem::get_int(self.ptr, "price").unwrap();
		qv.into_value().unwrap()
	}
	pub fn price_def_val(&self) -> i64{
		let qv = citem::get_int_def(self.ptr, "price").unwrap();
		qv.into_value().unwrap()
	}
	
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct PcListMItem {
	ptr : MItemPtr,
}
impl From<MItemPtr> for PcListMItem {
	fn from(ptr : MItemPtr) -> Self {
		Self{ ptr }
	}
}
impl PcListMItem {
	pub fn items(&mut self) -> MListPtr<ItemsMItem>{
		mitem::get_mil(self.ptr, "items").unwrap().unwrap()
	}
	pub fn name(&self) -> String{
		let qv = mitem::get_str(self.ptr, "name").unwrap();
		qv.into_value().unwrap()
	}
	pub fn name_def_val(&self) -> String{
		let qv = mitem::get_str_def(self.ptr, "name").unwrap();
		qv.into_value().unwrap()
	}
	pub fn set_name(&mut self, name : String){
		mitem::set_str(self.ptr, "name", Qv::Val(name));
	}
	
	
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ItemsMItem {
	ptr : MItemPtr,
}
impl From<MItemPtr> for ItemsMItem {
	fn from(ptr : MItemPtr) -> Self {
		Self{ ptr }
	}
}
impl ItemsMItem {
	pub fn get_enum(&self) -> ItemsEnum{
		let (list_name, _) = mitem::get_enum(self.ptr).unwrap();
		let p = if let Qv::Val(p) = mitem::get_ref(self.ptr, &list_name).unwrap(){ p } else { unreachable!() };
		ItemsEnum::new(&list_name,p)
	}
	pub fn get_enum_ids(&self) -> (String,String){
		mitem::get_enum(self.ptr).unwrap()
	}
	pub fn set_enum(&self, kind : ItemsKind){
		let (list_name, id) = kind.id();
		mitem::set_enum(self.ptr, list_name, id);
	}
}
pub enum ItemsEnum{ Sword(SwordCItem), Herb(HerbCItem), }
impl ItemsEnum{
	pub fn new(list_name : &str, ptr : CItemPtr) -> ItemsEnum{
		match list_name{
			"sword" => ItemsEnum::Sword(SwordCItem::from(ptr)),
			"herb" => ItemsEnum::Herb(HerbCItem::from(ptr)),
			_ => panic!("ItemsEnum there's no enum type named {}", &list_name),
		}
	}
}
pub enum ItemsKind{ Sword(SwordTableID), Herb(HerbTableID), }
impl ItemsKind{
	pub fn id(&self) -> (&'static str, &'static str){
		match self{
			ItemsKind::Sword(v) => ("sword", v.to_str()),
			ItemsKind::Herb(v) => ("herb", v.to_str()),
		}
	}
}

#[derive(Debug, PartialEq)]
pub struct HerbTable {
	ptr : TablePtr,
}
impl HerbTable {
	pub fn new(ptr : TablePtr) -> HerbTable{ HerbTable{ ptr } } 
	pub fn middle(&self) -> HerbCItem {
		let ptr = table::get_value(self.ptr, "middle").unwrap();
		HerbCItem::from(ptr)
	}
	pub fn low(&self) -> HerbCItem {
		let ptr = table::get_value(self.ptr, "low").unwrap();
		HerbCItem::from(ptr)
	}
	pub fn get_by_id(&self, id : HerbTableID) -> HerbCItem{
		match id{
			HerbTableID::Middle => self.middle(),
			HerbTableID::Low => self.low(),
		}
	}
}
#[repr(u64)] pub enum HerbTableID{ Middle, Low, }
impl HerbTableID{
	pub fn from_str(id : &str) -> Option<Self>{
		match id{
			"middle" => Some(Self::Middle),
			"low" => Some(Self::Low),
			_ =>{ None }
		}
	}
	pub fn from_num(id : usize) -> Self{
		match id{
			0 => Self::Middle,
			1 => Self::Low,
			_ => panic!("invalid ID num {} HerbTableID", id),
		}
	}
	pub fn len() -> usize{ 2 }
	pub fn to_num(&self) -> usize{
		match self{
			HerbTableID::Middle => 0,
			HerbTableID::Low => 1,
		}
	}
	pub fn metadata() -> &'static [&'static str]{
		&["middle", "low", ]
	}
	pub fn to_str(&self) -> &'static str{
		Self::metadata()[self.to_num()]
	}
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct HerbCItem {
	ptr : CItemPtr,
}
impl From<CItemPtr> for HerbCItem {
	fn from(ptr : CItemPtr) -> Self { Self{ ptr } }
}
impl HerbCItem {
	pub fn restore(&self) -> i64{
		let qv = citem::get_int(self.ptr, "restore").unwrap();
		qv.into_value().unwrap()
	}
	pub fn restore_def_val(&self) -> i64{
		let qv = citem::get_int_def(self.ptr, "restore").unwrap();
		qv.into_value().unwrap()
	}
	pub fn price(&self) -> i64{
		let qv = citem::get_int(self.ptr, "price").unwrap();
		qv.into_value().unwrap()
	}
	pub fn price_def_val(&self) -> i64{
		let qv = citem::get_int_def(self.ptr, "price").unwrap();
		qv.into_value().unwrap()
	}
	
}

