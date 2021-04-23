// use dochy::core::intf::*;
// use dochy::core::structs::*;
// #[derive(Debug, PartialEq)]
// pub struct RootIntf{
//     root : Box<RootObject>,
//     ptr : RootObjectPtr,
// }
// impl RootIntf{
//     pub fn new(obj : RootObject) -> RootIntf{
// 		let mut root = Box::new(obj);
// 		let ptr = RootObjectPtr::new(root.as_mut());
// 		RootIntf { root, ptr }
// 	}
//     pub unsafe fn root_obj_ref(&self) -> &RootObject{ self.root.as_ref() }
//     pub unsafe fn root_obj_ref_mut(&mut self) -> &mut RootObject{ self.root.as_mut() }
//
// 	pub fn name_testing(&self) -> NameTestingTable{
// 		let ans = root::get_table(self.ptr, "name_testing").unwrap();
// 		NameTestingTable::new(ans)
// 	}
// 	pub fn referent_a(&self) -> ReferentATable{
// 		let ans = root::get_table(self.ptr, "referentA").unwrap();
// 		ReferentATable::new(ans)
// 	}
// 	pub fn mlist(&mut self) -> MListPtr<MlistMItem>{
// 		root::get_mlist(self.ptr, "mlist").unwrap()
// 	}
// 	pub fn referent_b(&self) -> ReferentBTable{
// 		let ans = root::get_table(self.ptr, "referentB").unwrap();
// 		ReferentBTable::new(ans)
// 	}
// }
// #[derive(Debug, PartialEq)]
// pub struct NameTestingTable {
// 	ptr : TablePtr,
// }
// impl NameTestingTable {
// 	pub fn new(ptr : TablePtr) -> NameTestingTable{ NameTestingTable{ ptr } }
// 	pub unsafe fn huga_us(&self) -> NameTestingCItem {
// 		let ptr = table::get_value(self.ptr, "huga").unwrap();
// 		NameTestingCItem::from(ptr)
// 	}
// 	pub fn huga(&self) -> CItemConst<NameTestingCItem> {
// 		CItemConst::new(self.huga_us(), self)
// 	}
// 	pub unsafe fn get_by_id_us(&self, id : NameTestingTableID) -> NameTestingCItem{
// 		match id{
// 			NameTestingTableID::Huga => self.huga_us(),
// 		}
// 	}
// 	pub fn get_by_id(&self, id : NameTestingTableID) -> CItemConst<NameTestingCItem>{
// 		CItemConst::new(get_by_id(id), self)
// 	}
// }
// #[repr(u64)] pub enum NameTestingTableID{ Huga, }
// impl NameTestingTableID{
// 	pub fn from_str(id : &str) -> Option<Self>{
// 		match id{
// 			"huga" => Some(Self::Huga),
// 			_ =>{ None }
// 		}
// 	}
// 	pub fn from_num(id : usize) -> Self{
// 		match id{
// 			0 => Self::Huga,
// 			_ => panic!("invalid ID num {} NameTestingTableID", id),
// 		}
// 	}
// 	pub fn len() -> usize{ 1 }
// 	pub fn to_num(&self) -> usize{
// 		match self{
// 			NameTestingTableID::Huga => 0,
// 		}
// 	}
// 	pub fn metadata() -> &'static [&'static str]{
// 		&["huga", ]
// 	}
// 	pub fn to_str(&self) -> &'static str{
// 		Self::metadata()[self.to_num()]
// 	}
// }
// #[derive(Debug, PartialEq, Clone, Copy)]
// pub struct NameTestingCItem {
// 	ptr : CItemPtr,
// }
// impl From<CItemPtr> for NameTestingCItem {
// 	fn from(ptr : CItemPtr) -> Self { Self{ ptr } }
// }
// impl NameTestingCItem {
//
// }
//
// #[derive(Debug, PartialEq)]
// pub struct ReferentATable {
// 	ptr : TablePtr,
// }
// impl ReferentATable {
// 	pub fn new(ptr : TablePtr) -> ReferentATable{ ReferentATable{ ptr } }
// 	pub unsafe fn aa_us(&self) -> ReferentACItem {
// 		let ptr = table::get_value(self.ptr, "aa").unwrap();
// 		ReferentACItem::from(ptr)
// 	}
// 	pub fn aa(&self) -> CItemConst<ReferentACItem> {
// 		CItemConst::new(self.aa_us(), self)
// 	}
// 	pub unsafe fn ab_us(&self) -> ReferentACItem {
// 		let ptr = table::get_value(self.ptr, "ab").unwrap();
// 		ReferentACItem::from(ptr)
// 	}
// 	pub fn ab(&self) -> CItemConst<ReferentACItem> {
// 		CItemConst::new(self.ab_us(), self)
// 	}
// 	pub unsafe fn get_by_id_us(&self, id : ReferentATableID) -> ReferentACItem{
// 		match id{
// 			ReferentATableID::Aa => self.aa_us(),
// 			ReferentATableID::Ab => self.ab_us(),
// 		}
// 	}
// 	pub fn get_by_id(&self, id : ReferentATableID) -> CItemConst<ReferentACItem>{
// 		CItemConst::new(get_by_id(id), self)
// 	}
// }
// #[repr(u64)] pub enum ReferentATableID{ Aa, Ab, }
// impl ReferentATableID{
// 	pub fn from_str(id : &str) -> Option<Self>{
// 		match id{
// 			"aa" => Some(Self::Aa),
// 			"ab" => Some(Self::Ab),
// 			_ =>{ None }
// 		}
// 	}
// 	pub fn from_num(id : usize) -> Self{
// 		match id{
// 			0 => Self::Aa,
// 			1 => Self::Ab,
// 			_ => panic!("invalid ID num {} ReferentATableID", id),
// 		}
// 	}
// 	pub fn len() -> usize{ 2 }
// 	pub fn to_num(&self) -> usize{
// 		match self{
// 			ReferentATableID::Aa => 0,
// 			ReferentATableID::Ab => 1,
// 		}
// 	}
// 	pub fn metadata() -> &'static [&'static str]{
// 		&["aa", "ab", ]
// 	}
// 	pub fn to_str(&self) -> &'static str{
// 		Self::metadata()[self.to_num()]
// 	}
// }
// #[derive(Debug, PartialEq, Clone, Copy)]
// pub struct ReferentACItem {
// 	ptr : CItemPtr,
// }
// impl From<CItemPtr> for ReferentACItem {
// 	fn from(ptr : CItemPtr) -> Self { Self{ ptr } }
// }
// impl ReferentACItem {
// 	pub fn va(&self) -> i64{
// 		let qv = citem::get_int(self.ptr, "va").unwrap();
// 		qv.into_value().unwrap()
// 	}
// 	pub fn va_def_val(&self) -> i64{
// 		let qv = citem::get_int_def(self.ptr, "va").unwrap();
// 		qv.into_value().unwrap()
// 	}
// 	pub fn vc(&self) -> i64{
// 		let qv = citem::get_int(self.ptr, "vc").unwrap();
// 		qv.into_value().unwrap()
// 	}
// 	pub fn vc_def_val(&self) -> i64{
// 		let qv = citem::get_int_def(self.ptr, "vc").unwrap();
// 		qv.into_value().unwrap()
// 	}
//
// }
//
// #[derive(Debug, PartialEq, Clone, Copy)]
// pub struct MlistMItem {
// 	ptr : MItemPtr,
// }
// impl From<MItemPtr> for MlistMItem {
// 	fn from(ptr : MItemPtr) -> Self {
// 		Self{ ptr }
// 	}
// }
// impl MlistMItem {
// 	pub fn name_test(&self) -> i64{
// 		let qv = mitem::get_int(self.ptr, "name_test").unwrap();
// 		qv.into_value().unwrap()
// 	}
// 	pub fn name_test_def_val(&self) -> i64{
// 		let qv = mitem::get_int_def(self.ptr, "name_test").unwrap();
// 		qv.into_value().unwrap()
// 	}
// 	pub fn set_name_test(&mut self, name_test : i64){
// 		mitem::set_int(self.ptr, "name_test", Qv::Val(name_test));
// 	}
// 	pub fn vb(&self) -> NullOr<i64>{
// 		let qv = mitem::get_int(self.ptr, "vb").unwrap();
// 		NullOr::from_qv(qv).unwrap()
// 	}
// 	pub fn vb_def_val(&self) -> NullOr<i64>{
// 		let qv = mitem::get_int_def(self.ptr, "vb").unwrap();
// 		NullOr::from_qv(qv).unwrap()
// 	}
// 	pub fn set_vb(&mut self, vb : NullOr<i64>){
// 		mitem::set_int(self.ptr, "vb", vb.into_qv());
// 	}
// 	pub fn vc(&self) -> NullOr<i64>{
// 		let qv = mitem::get_int(self.ptr, "vc").unwrap();
// 		NullOr::from_qv(qv).unwrap()
// 	}
// 	pub fn vc_def_val(&self) -> NullOr<i64>{
// 		let qv = mitem::get_int_def(self.ptr, "vc").unwrap();
// 		NullOr::from_qv(qv).unwrap()
// 	}
// 	pub fn set_vc(&mut self, vc : NullOr<i64>){
// 		mitem::set_int(self.ptr, "vc", vc.into_qv());
// 	}
// 	pub fn va(&self) -> NullOr<i64>{
// 		let qv = mitem::get_int(self.ptr, "va").unwrap();
// 		NullOr::from_qv(qv).unwrap()
// 	}
// 	pub fn va_def_val(&self) -> NullOr<i64>{
// 		let qv = mitem::get_int_def(self.ptr, "va").unwrap();
// 		NullOr::from_qv(qv).unwrap()
// 	}
// 	pub fn set_va(&mut self, va : NullOr<i64>){
// 		mitem::set_int(self.ptr, "va", va.into_qv());
// 	}
// 	pub fn get_enum(&self) -> MlistEnum{
// 		let (list_name, _) = mitem::get_enum(self.ptr).unwrap();
// 		let p = if let Qv::Val(p) = mitem::get_ref(self.ptr, &list_name).unwrap(){ p } else { unreachable!() };
// 		MlistEnum::new(&list_name,p)
// 	}
// 	pub fn get_enum_ids(&self) -> (String,String){
// 		mitem::get_enum(self.ptr).unwrap()
// 	}
// 	pub fn set_enum(&self, kind : MlistKind){
// 		let (list_name, id) = kind.id();
// 		mitem::set_enum(self.ptr, list_name, id);
// 	}
// }
// pub enum MlistEnum{ ReferentB(ReferentBCItem), ReferentA(ReferentACItem), NameTesting(NameTestingCItem), }
// impl MlistEnum{
// 	pub fn new(list_name : &str, ptr : CItemPtr) -> MlistEnum{
// 		match list_name{
// 			"referentB" => MlistEnum::ReferentB(ReferentBCItem::from(ptr)),
// 			"referentA" => MlistEnum::ReferentA(ReferentACItem::from(ptr)),
// 			"name_testing" => MlistEnum::NameTesting(NameTestingCItem::from(ptr)),
// 			_ => panic!("MlistEnum there's no enum type named {}", &list_name),
// 		}
// 	}
// }
// pub enum MlistKind{ ReferentB(ReferentBTableID), ReferentA(ReferentATableID), NameTesting(NameTestingTableID), }
// impl MlistKind{
// 	pub fn id(&self) -> (&'static str, &'static str){
// 		match self{
// 			MlistKind::ReferentB(v) => ("referentB", v.to_str()),
// 			MlistKind::ReferentA(v) => ("referentA", v.to_str()),
// 			MlistKind::NameTesting(v) => ("name_testing", v.to_str()),
// 		}
// 	}
// }
//
// #[derive(Debug, PartialEq)]
// pub struct ReferentBTable {
// 	ptr : TablePtr,
// }
// impl ReferentBTable {
// 	pub fn new(ptr : TablePtr) -> ReferentBTable{ ReferentBTable{ ptr } }
// 	pub unsafe fn ba_us(&self) -> ReferentBCItem {
// 		let ptr = table::get_value(self.ptr, "ba").unwrap();
// 		ReferentBCItem::from(ptr)
// 	}
// 	pub fn ba(&self) -> CItemConst<ReferentBCItem> {
// 		CItemConst::new(self.ba_us(), self)
// 	}
// 	pub unsafe fn bb_us(&self) -> ReferentBCItem {
// 		let ptr = table::get_value(self.ptr, "bb").unwrap();
// 		ReferentBCItem::from(ptr)
// 	}
// 	pub fn bb(&self) -> CItemConst<ReferentBCItem> {
// 		CItemConst::new(self.bb_us(), self)
// 	}
// 	pub unsafe fn get_by_id_us(&self, id : ReferentBTableID) -> ReferentBCItem{
// 		match id{
// 			ReferentBTableID::Ba => self.ba_us(),
// 			ReferentBTableID::Bb => self.bb_us(),
// 		}
// 	}
// 	pub fn get_by_id(&self, id : ReferentBTableID) -> CItemConst<ReferentBCItem>{
// 		CItemConst::new(get_by_id(id), self)
// 	}
// }
// #[repr(u64)] pub enum ReferentBTableID{ Ba, Bb, }
// impl ReferentBTableID{
// 	pub fn from_str(id : &str) -> Option<Self>{
// 		match id{
// 			"ba" => Some(Self::Ba),
// 			"bb" => Some(Self::Bb),
// 			_ =>{ None }
// 		}
// 	}
// 	pub fn from_num(id : usize) -> Self{
// 		match id{
// 			0 => Self::Ba,
// 			1 => Self::Bb,
// 			_ => panic!("invalid ID num {} ReferentBTableID", id),
// 		}
// 	}
// 	pub fn len() -> usize{ 2 }
// 	pub fn to_num(&self) -> usize{
// 		match self{
// 			ReferentBTableID::Ba => 0,
// 			ReferentBTableID::Bb => 1,
// 		}
// 	}
// 	pub fn metadata() -> &'static [&'static str]{
// 		&["ba", "bb", ]
// 	}
// 	pub fn to_str(&self) -> &'static str{
// 		Self::metadata()[self.to_num()]
// 	}
// }
// #[derive(Debug, PartialEq, Clone, Copy)]
// pub struct ReferentBCItem {
// 	ptr : CItemPtr,
// }
// impl From<CItemPtr> for ReferentBCItem {
// 	fn from(ptr : CItemPtr) -> Self { Self{ ptr } }
// }
// impl ReferentBCItem {
// 	pub fn vb(&self) -> i64{
// 		let qv = citem::get_int(self.ptr, "vb").unwrap();
// 		qv.into_value().unwrap()
// 	}
// 	pub fn vb_def_val(&self) -> i64{
// 		let qv = citem::get_int_def(self.ptr, "vb").unwrap();
// 		qv.into_value().unwrap()
// 	}
// 	pub fn vc(&self) -> i64{
// 		let qv = citem::get_int(self.ptr, "vc").unwrap();
// 		qv.into_value().unwrap()
// 	}
// 	pub fn vc_def_val(&self) -> i64{
// 		let qv = citem::get_int_def(self.ptr, "vc").unwrap();
// 		qv.into_value().unwrap()
// 	}
//
// }
//
