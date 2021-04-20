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

	pub fn class(&self) -> ClassTable{
		let ans = root::get_table(self.ptr, "class").unwrap();
		ClassTable::new(ans)
	}
	pub fn pc_list(&mut self) -> MListPtr<PcListMItem>{
		root::get_mlist(self.ptr, "pcList").unwrap()
	}
	pub fn race(&self) -> RaceTable{
		let ans = root::get_table(self.ptr, "race").unwrap();
		RaceTable::new(ans)
	}
}
#[derive(Debug, PartialEq)]
pub struct ClassTable {
	ptr : TablePtr,
}
impl ClassTable {
	pub fn new(ptr : TablePtr) -> ClassTable{ ClassTable{ ptr } } 
	pub fn mage(&self) -> ClassCItem {
		let ptr = table::get_value(self.ptr, "mage").unwrap();
		ClassCItem::from(ptr)
	}
	pub fn fighter(&self) -> ClassCItem {
		let ptr = table::get_value(self.ptr, "fighter").unwrap();
		ClassCItem::from(ptr)
	}
	pub fn get_by_id(&self, id : ClassTableID) -> ClassCItem{
		match id{
			ClassTableID::Mage => self.mage(),
			ClassTableID::Fighter => self.fighter(),
		}
	}
}
#[repr(u64)] pub enum ClassTableID{ Mage, Fighter, }
impl ClassTableID{
	pub fn from_str(id : &str) -> Option<Self>{
		match id{
			"mage" => Some(Self::Mage),
			"fighter" => Some(Self::Fighter),
			_ =>{ None }
		}
	}
	pub fn from_num(id : usize) -> Self{
		match id{
			0 => Self::Mage,
			1 => Self::Fighter,
			_ => panic!("invalid ID num {} ClassTableID", id),
		}
	}
	pub fn len() -> usize{ 2 }
	pub fn to_num(&self) -> usize{
		match self{
			ClassTableID::Mage => 0,
			ClassTableID::Fighter => 1,
		}
	}
	pub fn metadata() -> &'static [&'static str]{
		&["mage", "fighter", ]
	}
	pub fn to_str(&self) -> &'static str{
		Self::metadata()[self.to_num()]
	}
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ClassCItem {
	ptr : CItemPtr,
}
impl From<CItemPtr> for ClassCItem {
	fn from(ptr : CItemPtr) -> Self { Self{ ptr } }
}
impl ClassCItem {
	pub fn magic(&self) -> i64{
		let qv = citem::get_int(self.ptr, "magic").unwrap();
		qv.into_value().unwrap()
	}
	pub fn magic_def_val(&self) -> i64{
		let qv = citem::get_int_def(self.ptr, "magic").unwrap();
		qv.into_value().unwrap()
	}
	pub fn attack(&self) -> i64{
		let qv = citem::get_int(self.ptr, "attack").unwrap();
		qv.into_value().unwrap()
	}
	pub fn attack_def_val(&self) -> i64{
		let qv = citem::get_int_def(self.ptr, "attack").unwrap();
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
	pub fn ref_class(&self) -> ClassCItem{
		let qv = mitem::get_ref(self.ptr, "class").unwrap();
		ClassCItem::from(qv.into_value().unwrap())
	}
	pub fn ref_id_class(&self) -> String{
		let qv = mitem::get_ref_id(self.ptr, "class").unwrap();
		qv.into_value().unwrap()
	}
	pub fn ref_race(&self) -> RaceCItem{
		let qv = mitem::get_ref(self.ptr, "race").unwrap();
		RaceCItem::from(qv.into_value().unwrap())
	}
	pub fn ref_id_race(&self) -> String{
		let qv = mitem::get_ref_id(self.ptr, "race").unwrap();
		qv.into_value().unwrap()
	}
	pub fn set_ref_class(&self, id : ClassTableID){
		mitem::set_ref(self.ptr, "class", Qv::Val(id.to_str().to_string()));
	}
	pub fn set_ref_race(&self, id : RaceTableID){
		mitem::set_ref(self.ptr, "race", Qv::Val(id.to_str().to_string()));
	}
}

#[derive(Debug, PartialEq)]
pub struct RaceTable {
	ptr : TablePtr,
}
impl RaceTable {
	pub fn new(ptr : TablePtr) -> RaceTable{ RaceTable{ ptr } } 
	pub fn dwarf(&self) -> RaceCItem {
		let ptr = table::get_value(self.ptr, "dwarf").unwrap();
		RaceCItem::from(ptr)
	}
	pub fn elf(&self) -> RaceCItem {
		let ptr = table::get_value(self.ptr, "elf").unwrap();
		RaceCItem::from(ptr)
	}
	pub fn get_by_id(&self, id : RaceTableID) -> RaceCItem{
		match id{
			RaceTableID::Dwarf => self.dwarf(),
			RaceTableID::Elf => self.elf(),
		}
	}
}
#[repr(u64)] pub enum RaceTableID{ Dwarf, Elf, }
impl RaceTableID{
	pub fn from_str(id : &str) -> Option<Self>{
		match id{
			"dwarf" => Some(Self::Dwarf),
			"elf" => Some(Self::Elf),
			_ =>{ None }
		}
	}
	pub fn from_num(id : usize) -> Self{
		match id{
			0 => Self::Dwarf,
			1 => Self::Elf,
			_ => panic!("invalid ID num {} RaceTableID", id),
		}
	}
	pub fn len() -> usize{ 2 }
	pub fn to_num(&self) -> usize{
		match self{
			RaceTableID::Dwarf => 0,
			RaceTableID::Elf => 1,
		}
	}
	pub fn metadata() -> &'static [&'static str]{
		&["dwarf", "elf", ]
	}
	pub fn to_str(&self) -> &'static str{
		Self::metadata()[self.to_num()]
	}
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct RaceCItem {
	ptr : CItemPtr,
}
impl From<CItemPtr> for RaceCItem {
	fn from(ptr : CItemPtr) -> Self { Self{ ptr } }
}
impl RaceCItem {
	pub fn strength(&self) -> i64{
		let qv = citem::get_int(self.ptr, "strength").unwrap();
		qv.into_value().unwrap()
	}
	pub fn strength_def_val(&self) -> i64{
		let qv = citem::get_int_def(self.ptr, "strength").unwrap();
		qv.into_value().unwrap()
	}
	pub fn intelligence(&self) -> i64{
		let qv = citem::get_int(self.ptr, "intelligence").unwrap();
		qv.into_value().unwrap()
	}
	pub fn intelligence_def_val(&self) -> i64{
		let qv = citem::get_int_def(self.ptr, "intelligence").unwrap();
		qv.into_value().unwrap()
	}
	
}
