// use crate::core::intf::*;
// use crate::core::structs::*;
// unsafe impl Send for RootIntf{}
// #[derive(Debug, PartialEq)]
// pub(crate ) struct RootIntf{
//     root : Box<RootObject>,
//     ptr : RootObjectPtr,
// }
// impl RootIntf{
//     pub(crate ) fn new(obj : RootObject) -> RootIntf{
// 		let mut root = Box::new(obj);
// 		let ptr = RootObjectPtr::new(root.as_mut());
// 		RootIntf { root, ptr }
// 	}
//     pub(crate ) unsafe fn root_obj_ref(&self) -> &RootObject{ self.root.as_ref() }
//     pub(crate ) unsafe fn root_obj_ref_mut(&mut self) -> &mut RootObject{ self.root.as_mut() }
//
// 	pub(crate ) fn pugya_int(&self) -> UndefOr<i64>{
// 		let qv = root::get_int(self.ptr, "pugyaInt").unwrap();
// 		UndefOr::from_qv(qv).unwrap()
// 	}
// 	pub(crate ) fn set_pugya_int(&mut self, pugya_int : UndefOr<i64>){
// 		root::set_int(self.ptr, "pugyaInt", pugya_int.into_qv());
// 	}
// 	pub(crate ) fn hoge_bool(&self) -> bool{
// 		let qv = root::get_bool(self.ptr, "hogeBool").unwrap();
// 		qv.into_value().unwrap()
// 	}
// 	pub(crate ) fn set_hoge_bool(&mut self, hoge_bool : bool){
// 		root::set_bool(self.ptr, "hogeBool", Qv::Val(hoge_bool));
// 	}
// 	pub(crate ) fn at_huga_number(&self) -> i64{
// 		let qv = root::get_int(self.ptr, "@HugaNumber").unwrap();
// 		qv.into_value().unwrap()
// 	}
// 	pub(crate ) fn set_at_huga_number(&mut self, at_huga_number : i64){
// 		root::set_int(self.ptr, "@HugaNumber", Qv::Val(at_huga_number));
// 	}
// 	pub(crate ) fn ini_item_list(&self) -> CListPtr<IniItemListCItem>{
// 		root::get_clist(self.ptr, "iniItemList").unwrap()
// 	}
// 	pub(crate ) fn int_array(&self) -> Vec<i64>{
// 		let qv = root::get_int_array(self.ptr, "intArray").unwrap();
// 		qv.into_value().unwrap()
// 	}
// 	pub(crate ) fn set_int_array(&mut self, int_array : Vec<i64>){
// 		root::set_int_array(self.ptr, "intArray", Qv::Val(int_array));
// 	}
// 	pub(crate ) fn hego_float(&self) -> NullOr<f64>{
// 		let qv = root::get_float(self.ptr, "hegoFloat").unwrap();
// 		NullOr::from_qv(qv).unwrap()
// 	}
// 	pub(crate ) fn set_hego_float(&mut self, hego_float : NullOr<f64>){
// 		root::set_float(self.ptr, "hegoFloat", hego_float.into_qv());
// 	}
// 	pub(crate ) fn new_name(&self) -> UndefOr<String>{
// 		let qv = root::get_str(self.ptr, "newName").unwrap();
// 		UndefOr::from_qv(qv).unwrap()
// 	}
// 	pub(crate ) fn set_new_name(&mut self, new_name : UndefOr<String>){
// 		root::set_str(self.ptr, "newName", new_name.into_qv());
// 	}
// 	pub(crate ) fn new_name2(&self) -> UndefOr<i64>{
// 		let qv = root::get_int(self.ptr, "newName2").unwrap();
// 		UndefOr::from_qv(qv).unwrap()
// 	}
// 	pub(crate ) fn set_new_name2(&mut self, new_name2 : UndefOr<i64>){
// 		root::set_int(self.ptr, "newName2", new_name2.into_qv());
// 	}
// 	pub(crate ) fn hoge_float(&self) -> f64{
// 		let qv = root::get_float(self.ptr, "hogeFloat").unwrap();
// 		qv.into_value().unwrap()
// 	}
// 	pub(crate ) fn set_hoge_float(&mut self, hoge_float : f64){
// 		root::set_float(self.ptr, "hogeFloat", Qv::Val(hoge_float));
// 	}
// 	pub(crate ) fn weapon(&self) -> WeaponTable{
// 		let ans = root::get_table(self.ptr, "weapon").unwrap();
// 		WeaponTable::new(ans)
// 	}
// 	pub(crate ) fn float_array(&self) -> Vec<f64>{
// 		let qv = root::get_float_array(self.ptr, "floatArray").unwrap();
// 		qv.into_value().unwrap()
// 	}
// 	pub(crate ) fn set_float_array(&mut self, float_array : Vec<f64>){
// 		root::set_float_array(self.ptr, "floatArray", Qv::Val(float_array));
// 	}
// 	pub(crate ) fn nullable_int_array(&self) -> NullOr<Vec<i64>>{
// 		let qv = root::get_int_array(self.ptr, "nullableIntArray").unwrap();
// 		NullOr::from_qv(qv).unwrap()
// 	}
// 	pub(crate ) fn set_nullable_int_array(&mut self, nullable_int_array : NullOr<Vec<i64>>){
// 		root::set_int_array(self.ptr, "nullableIntArray", nullable_int_array.into_qv());
// 	}
// 	pub(crate ) fn hoge_list(&self) -> HogeListTable{
// 		let ans = root::get_table(self.ptr, "hogeList").unwrap();
// 		HogeListTable::new(ans)
// 	}
// 	pub(crate ) fn hoge_int(&self) -> i64{
// 		let qv = root::get_int(self.ptr, "hogeInt").unwrap();
// 		qv.into_value().unwrap()
// 	}
// 	pub(crate ) fn set_hoge_int(&mut self, hoge_int : i64){
// 		root::set_int(self.ptr, "hogeInt", Qv::Val(hoge_int));
// 	}
// 	pub(crate ) fn usable(&self) -> UsableTable{
// 		let ans = root::get_table(self.ptr, "usable").unwrap();
// 		UsableTable::new(ans)
// 	}
// 	pub(crate ) fn item_list3(&self) -> CListPtr<ItemList3CItem>{
// 		root::get_clist(self.ptr, "itemList3").unwrap()
// 	}
// 	pub(crate ) fn mut1(&mut self) -> MListPtr<Mut1MItem>{
// 		root::get_mlist(self.ptr, "mut1").unwrap()
// 	}
// 	pub(crate ) fn hego_int(&self) -> NullOr<i64>{
// 		let qv = root::get_int(self.ptr, "hegoInt").unwrap();
// 		NullOr::from_qv(qv).unwrap()
// 	}
// 	pub(crate ) fn set_hego_int(&mut self, hego_int : NullOr<i64>){
// 		root::set_int(self.ptr, "hegoInt", hego_int.into_qv());
// 	}
// 	pub(crate ) fn huga_list(&self) -> HugaListTable{
// 		let ans = root::get_table(self.ptr, "hugaList").unwrap();
// 		HugaListTable::new(ans)
// 	}
// 	pub(crate ) fn dim2_list(&self) -> Dim2ListTable{
// 		let ans = root::get_table(self.ptr, "dim2List").unwrap();
// 		Dim2ListTable::new(ans)
// 	}
// 	pub(crate ) fn unko_list(&self) -> UnkoListTable{
// 		let ans = root::get_table(self.ptr, "unkoList").unwrap();
// 		UnkoListTable::new(ans)
// 	}
// 	pub(crate ) fn hoge_string(&self) -> String{
// 		let qv = root::get_str(self.ptr, "hogeString").unwrap();
// 		qv.into_value().unwrap()
// 	}
// 	pub(crate ) fn set_hoge_string(&mut self, hoge_string : String){
// 		root::set_str(self.ptr, "hogeString", Qv::Val(hoge_string));
// 	}
// 	pub(crate ) fn hego_list(&self) -> HegoListTable{
// 		let ans = root::get_table(self.ptr, "hegoList").unwrap();
// 		HegoListTable::new(ans)
// 	}
// 	pub(crate ) fn mut2(&mut self) -> MListPtr<Mut2MItem>{
// 		root::get_mlist(self.ptr, "mut2").unwrap()
// 	}
// 	pub(crate ) fn old_name2_old(&self) -> NullOr<i64>{
// 		let qv = root::get_int(self.ptr, "oldName2").unwrap();
// 		NullOr::from_qv(qv).unwrap()
// 	}
// 	pub(crate ) fn set_old_name2_old(&mut self, old_name2 : NullOr<i64>){
// 		root::set_int(self.ptr, "oldName2", old_name2.into_qv());
// 	}
// 	pub(crate ) fn pugya_int2(&self) -> Qv<i64>{
// 		let qv = root::get_int(self.ptr, "pugyaInt2").unwrap();
// 		qv
// 	}
// 	pub(crate ) fn set_pugya_int2(&mut self, pugya_int2 : Qv<i64>){
// 		root::set_int(self.ptr, "pugyaInt2", pugya_int2.into_qv());
// 	}
// 	pub(crate ) fn enum_list(&self) -> CListPtr<EnumListCItem>{
// 		root::get_clist(self.ptr, "enumList").unwrap()
// 	}
// 	pub(crate ) fn empty_int_array(&self) -> Vec<i64>{
// 		let qv = root::get_int_array(self.ptr, "emptyIntArray").unwrap();
// 		qv.into_value().unwrap()
// 	}
// 	pub(crate ) fn set_empty_int_array(&mut self, empty_int_array : Vec<i64>){
// 		root::set_int_array(self.ptr, "emptyIntArray", Qv::Val(empty_int_array));
// 	}
// 	pub(crate ) fn old_name_old(&self) -> NullOr<String>{
// 		let qv = root::get_str(self.ptr, "oldName").unwrap();
// 		NullOr::from_qv(qv).unwrap()
// 	}
// 	pub(crate ) fn set_old_name_old(&mut self, old_name : NullOr<String>){
// 		root::set_str(self.ptr, "oldName", old_name.into_qv());
// 	}
// }
// #[derive(Debug, PartialEq)]
// pub(crate ) struct IniItemListCItem {
// 	ptr : CItemPtr,
// }
// impl From<CItemPtr> for IniItemListCItem {
// 	fn from(ptr : CItemPtr) -> Self { Self{ ptr } }
// }
// impl IniItemListCItem {
// 	pub(crate ) fn atk(&self) -> NullOr<i64>{
// 		let qv = citem::get_int(self.ptr, "atk").unwrap();
// 		NullOr::from_qv(qv).unwrap()
// 	}
// 	pub(crate ) fn ref_weapon(&self) -> WeaponCItem{
// 		let qv = citem::get_ref(self.ptr, "weapon").unwrap();
// 		WeaponCItem::from(qv.into_value().unwrap())
// 	}
// 	pub(crate ) fn ref_id_weapon(&self) -> String{
// 		let qv = citem::get_ref_id(self.ptr, "weapon").unwrap();
// 		qv.into_value().unwrap()
// 	}
// }
//
// #[derive(Debug, PartialEq)]
// pub(crate ) struct WeaponTable {
// 	ptr : TablePtr,
// }
// impl WeaponTable {
// 	pub(crate ) fn new(ptr : TablePtr) -> WeaponTable{ WeaponTable{ ptr } }
// 	pub(crate ) fn katana(&self) -> WeaponCItem {
// 		let ptr = table::get_value(self.ptr, "katana").unwrap();
// 		WeaponCItem::from(ptr)
// 	}
// 	pub(crate ) fn doutanuki(&self) -> WeaponCItem {
// 		let ptr = table::get_value(self.ptr, "doutanuki").unwrap();
// 		WeaponCItem::from(ptr)
// 	}
// 	pub(crate ) fn from_id(&self, id : WeaponTableID) -> WeaponCItem{
// 		match id{
// 			WeaponTableID::Katana => self.katana(),
// 			WeaponTableID::Doutanuki => self.doutanuki(),
// 		}
// 	}
// }
// #[repr(u64)] pub(crate ) enum WeaponTableID{ Katana, Doutanuki, }
// impl WeaponTableID{
// 	pub(crate ) fn from_str(id : &str) -> Option<Self>{
// 		match id{
// 			"katana" => Some(Self::Katana),
// 			"doutanuki" => Some(Self::Doutanuki),
// 			_ =>{ None }
// 		}
// 	}
// 	pub(crate ) fn from_num(id : usize) -> Self{
// 		match id{
// 			0 => Self::Katana,
// 			1 => Self::Doutanuki,
// 			_ => panic!("invalid ID num {} WeaponTableID", id),
// 		}
// 	}
// 	pub(crate ) fn len() -> usize{ 2 }
// 	pub(crate ) fn to_num(&self) -> usize{
// 		match self{
// 			WeaponTableID::Katana => 0,
// 			WeaponTableID::Doutanuki => 1,
// 		}
// 	}
// 	pub(crate ) fn metadata() -> &'static [&'static str]{
// 		&["katana", "doutanuki", ]
// 	}
// 	pub(crate ) fn to_str(&self) -> &'static str{
// 		Self::metadata()[self.to_num()]
// 	}
// }
// #[derive(Debug, PartialEq)]
// pub(crate ) struct WeaponCItem {
// 	ptr : CItemPtr,
// }
// impl From<CItemPtr> for WeaponCItem {
// 	fn from(ptr : CItemPtr) -> Self { Self{ ptr } }
// }
// impl WeaponCItem {
// 	pub(crate ) fn atk(&self) -> i64{
// 		let qv = citem::get_int(self.ptr, "atk").unwrap();
// 		qv.into_value().unwrap()
// 	}
//
// }
//
// #[derive(Debug, PartialEq)]
// pub(crate ) struct HogeListTable {
// 	ptr : TablePtr,
// }
// impl HogeListTable {
// 	pub(crate ) fn new(ptr : TablePtr) -> HogeListTable{ HogeListTable{ ptr } }
// 	pub(crate ) fn hogehoge(&self) -> HogeListCItem {
// 		let ptr = table::get_value(self.ptr, "hogehoge").unwrap();
// 		HogeListCItem::from(ptr)
// 	}
// 	pub(crate ) fn from_id(&self, id : HogeListTableID) -> HogeListCItem{
// 		match id{
// 			HogeListTableID::Hogehoge => self.hogehoge(),
// 		}
// 	}
// }
// #[repr(u64)] pub(crate ) enum HogeListTableID{ Hogehoge, }
// impl HogeListTableID{
// 	pub(crate ) fn from_str(id : &str) -> Option<Self>{
// 		match id{
// 			"hogehoge" => Some(Self::Hogehoge),
// 			_ =>{ None }
// 		}
// 	}
// 	pub(crate ) fn from_num(id : usize) -> Self{
// 		match id{
// 			0 => Self::Hogehoge,
// 			_ => panic!("invalid ID num {} HogeListTableID", id),
// 		}
// 	}
// 	pub(crate ) fn len() -> usize{ 1 }
// 	pub(crate ) fn to_num(&self) -> usize{
// 		match self{
// 			HogeListTableID::Hogehoge => 0,
// 		}
// 	}
// 	pub(crate ) fn metadata() -> &'static [&'static str]{
// 		&["hogehoge", ]
// 	}
// 	pub(crate ) fn to_str(&self) -> &'static str{
// 		Self::metadata()[self.to_num()]
// 	}
// }
// #[derive(Debug, PartialEq)]
// pub(crate ) struct HogeListCItem {
// 	ptr : CItemPtr,
// }
// impl From<CItemPtr> for HogeListCItem {
// 	fn from(ptr : CItemPtr) -> Self { Self{ ptr } }
// }
// impl HogeListCItem {
// 	pub(crate ) fn mem(&self) -> String{
// 		let qv = citem::get_str(self.ptr, "mem").unwrap();
// 		qv.into_value().unwrap()
// 	}
//
// }
//
// #[derive(Debug, PartialEq)]
// pub(crate ) struct UsableTable {
// 	ptr : TablePtr,
// }
// impl UsableTable {
// 	pub(crate ) fn new(ptr : TablePtr) -> UsableTable{ UsableTable{ ptr } }
// 	pub(crate ) fn yakusou(&self) -> UsableCItem {
// 		let ptr = table::get_value(self.ptr, "yakusou").unwrap();
// 		UsableCItem::from(ptr)
// 	}
// 	pub(crate ) fn dokukesisou(&self) -> UsableCItem {
// 		let ptr = table::get_value(self.ptr, "dokukesisou").unwrap();
// 		UsableCItem::from(ptr)
// 	}
// 	pub(crate ) fn from_id(&self, id : UsableTableID) -> UsableCItem{
// 		match id{
// 			UsableTableID::Yakusou => self.yakusou(),
// 			UsableTableID::Dokukesisou => self.dokukesisou(),
// 		}
// 	}
// }
// #[repr(u64)] pub(crate ) enum UsableTableID{ Yakusou, Dokukesisou, }
// impl UsableTableID{
// 	pub(crate ) fn from_str(id : &str) -> Option<Self>{
// 		match id{
// 			"yakusou" => Some(Self::Yakusou),
// 			"dokukesisou" => Some(Self::Dokukesisou),
// 			_ =>{ None }
// 		}
// 	}
// 	pub(crate ) fn from_num(id : usize) -> Self{
// 		match id{
// 			0 => Self::Yakusou,
// 			1 => Self::Dokukesisou,
// 			_ => panic!("invalid ID num {} UsableTableID", id),
// 		}
// 	}
// 	pub(crate ) fn len() -> usize{ 2 }
// 	pub(crate ) fn to_num(&self) -> usize{
// 		match self{
// 			UsableTableID::Yakusou => 0,
// 			UsableTableID::Dokukesisou => 1,
// 		}
// 	}
// 	pub(crate ) fn metadata() -> &'static [&'static str]{
// 		&["yakusou", "dokukesisou", ]
// 	}
// 	pub(crate ) fn to_str(&self) -> &'static str{
// 		Self::metadata()[self.to_num()]
// 	}
// }
// #[derive(Debug, PartialEq)]
// pub(crate ) struct UsableCItem {
// 	ptr : CItemPtr,
// }
// impl From<CItemPtr> for UsableCItem {
// 	fn from(ptr : CItemPtr) -> Self { Self{ ptr } }
// }
// impl UsableCItem {
// 	pub(crate ) fn num(&self) -> i64{
// 		let qv = citem::get_int(self.ptr, "num").unwrap();
// 		qv.into_value().unwrap()
// 	}
//
// }
//
// #[derive(Debug, PartialEq)]
// pub(crate ) struct ItemList3CItem {
// 	ptr : CItemPtr,
// }
// impl From<CItemPtr> for ItemList3CItem {
// 	fn from(ptr : CItemPtr) -> Self { Self{ ptr } }
// }
// impl ItemList3CItem {
// 	pub(crate ) fn mem_override(&self) -> NullOr<String>{
// 		let qv = citem::get_str(self.ptr, "memOverride").unwrap();
// 		NullOr::from_qv(qv).unwrap()
// 	}
// 	pub(crate ) fn ref_hoge_list(&self) -> HogeListCItem{
// 		let qv = citem::get_ref(self.ptr, "hogeList").unwrap();
// 		HogeListCItem::from(qv.into_value().unwrap())
// 	}
// 	pub(crate ) fn ref_id_hoge_list(&self) -> String{
// 		let qv = citem::get_ref_id(self.ptr, "hogeList").unwrap();
// 		qv.into_value().unwrap()
// 	}
// 	pub(crate ) fn ref_hego_list(&self) -> HegoListCItem{
// 		let qv = citem::get_ref(self.ptr, "hegoList").unwrap();
// 		HegoListCItem::from(qv.into_value().unwrap())
// 	}
// 	pub(crate ) fn ref_id_hego_list(&self) -> String{
// 		let qv = citem::get_ref_id(self.ptr, "hegoList").unwrap();
// 		qv.into_value().unwrap()
// 	}
// 	pub(crate ) fn ref_huga_list(&self) -> NullOr<HugaListCItem>{
// 		let qv = citem::get_ref(self.ptr, "hugaList").unwrap();
// 		NullOr::from_qv(qv).unwrap().map(|p| HugaListCItem::from(*p))
// 	}
// 	pub(crate ) fn ref_id_huga_list(&self) -> NullOr<String>{
// 		let qv = citem::get_ref_id(self.ptr, "hugaList").unwrap();
// 		NullOr::from_qv(qv).unwrap()
// 	}
// }
//
// #[derive(Debug, PartialEq)]
// pub(crate ) struct Mut1MItem {
// 	ptr : MItemPtr,
// }
// impl From<MItemPtr> for Mut1MItem {
// 	fn from(ptr : MItemPtr) -> Self {
// 		Self{ ptr }
// 	}
// }
// impl Mut1MItem {
// 	pub(crate ) fn inner_mut_list(&mut self) -> MListPtr<InnerMutListMItem>{
// 		mitem::get_mil(self.ptr, "innerMutList").unwrap().unwrap()
// 	}
// 	pub(crate ) fn some_name(&self) -> String{
// 		let qv = mitem::get_str(self.ptr, "someName").unwrap();
// 		qv.into_value().unwrap()
// 	}
// 	pub(crate ) fn set_some_name(&mut self, some_name : String){
// 		mitem::set_str(self.ptr, "someName", Qv::Val(some_name));
// 	}
//
//
// }
// #[derive(Debug, PartialEq)]
// pub(crate ) struct InnerMutListMItem {
// 	ptr : MItemPtr,
// }
// impl From<MItemPtr> for InnerMutListMItem {
// 	fn from(ptr : MItemPtr) -> Self {
// 		Self{ ptr }
// 	}
// }
// impl InnerMutListMItem {
// 	pub(crate ) fn inner_list_mem(&self) -> i64{
// 		let qv = mitem::get_int(self.ptr, "innerListMem").unwrap();
// 		qv.into_value().unwrap()
// 	}
// 	pub(crate ) fn set_inner_list_mem(&mut self, inner_list_mem : i64){
// 		mitem::set_int(self.ptr, "innerListMem", Qv::Val(inner_list_mem));
// 	}
// 	pub(crate ) fn ref_hego_list(&self) -> HegoListCItem{
// 		let qv = mitem::get_ref(self.ptr, "hegoList").unwrap();
// 		HegoListCItem::from(qv.into_value().unwrap())
// 	}
// 	pub(crate ) fn ref_id_hego_list(&self) -> String{
// 		let qv = mitem::get_ref_id(self.ptr, "hegoList").unwrap();
// 		qv.into_value().unwrap()
// 	}
// 	pub(crate ) fn set_ref_hego_list(&self, id : HegoListTableID){
// 		mitem::set_ref(self.ptr, "hegoList", Qv::Val(id.to_str().to_string()));
// 	}
// }
//
// #[derive(Debug, PartialEq)]
// pub(crate ) struct HugaListTable {
// 	ptr : TablePtr,
// }
// impl HugaListTable {
// 	pub(crate ) fn new(ptr : TablePtr) -> HugaListTable{ HugaListTable{ ptr } }
// 	pub(crate ) fn hugahuga(&self) -> HugaListCItem {
// 		let ptr = table::get_value(self.ptr, "hugahuga").unwrap();
// 		HugaListCItem::from(ptr)
// 	}
// 	pub(crate ) fn from_id(&self, id : HugaListTableID) -> HugaListCItem{
// 		match id{
// 			HugaListTableID::Hugahuga => self.hugahuga(),
// 		}
// 	}
// }
// #[repr(u64)] pub(crate ) enum HugaListTableID{ Hugahuga, }
// impl HugaListTableID{
// 	pub(crate ) fn from_str(id : &str) -> Option<Self>{
// 		match id{
// 			"hugahuga" => Some(Self::Hugahuga),
// 			_ =>{ None }
// 		}
// 	}
// 	pub(crate ) fn from_num(id : usize) -> Self{
// 		match id{
// 			0 => Self::Hugahuga,
// 			_ => panic!("invalid ID num {} HugaListTableID", id),
// 		}
// 	}
// 	pub(crate ) fn len() -> usize{ 1 }
// 	pub(crate ) fn to_num(&self) -> usize{
// 		match self{
// 			HugaListTableID::Hugahuga => 0,
// 		}
// 	}
// 	pub(crate ) fn metadata() -> &'static [&'static str]{
// 		&["hugahuga", ]
// 	}
// 	pub(crate ) fn to_str(&self) -> &'static str{
// 		Self::metadata()[self.to_num()]
// 	}
// }
// #[derive(Debug, PartialEq)]
// pub(crate ) struct HugaListCItem {
// 	ptr : CItemPtr,
// }
// impl From<CItemPtr> for HugaListCItem {
// 	fn from(ptr : CItemPtr) -> Self { Self{ ptr } }
// }
// impl HugaListCItem {
//
// }
//
// #[derive(Debug, PartialEq)]
// pub(crate ) struct Dim2ListTable {
// 	ptr : TablePtr,
// }
// impl Dim2ListTable {
// 	pub(crate ) fn new(ptr : TablePtr) -> Dim2ListTable{ Dim2ListTable{ ptr } }
// 	pub(crate ) fn item3(&self) -> Dim2ListCItem {
// 		let ptr = table::get_value(self.ptr, "item3").unwrap();
// 		Dim2ListCItem::from(ptr)
// 	}
// 	pub(crate ) fn item2(&self) -> Dim2ListCItem {
// 		let ptr = table::get_value(self.ptr, "item2").unwrap();
// 		Dim2ListCItem::from(ptr)
// 	}
// 	pub(crate ) fn item1(&self) -> Dim2ListCItem {
// 		let ptr = table::get_value(self.ptr, "item1").unwrap();
// 		Dim2ListCItem::from(ptr)
// 	}
// 	pub(crate ) fn from_id(&self, id : Dim2ListTableID) -> Dim2ListCItem{
// 		match id{
// 			Dim2ListTableID::Item3 => self.item3(),
// 			Dim2ListTableID::Item2 => self.item2(),
// 			Dim2ListTableID::Item1 => self.item1(),
// 		}
// 	}
// }
// #[repr(u64)] pub(crate ) enum Dim2ListTableID{ Item3, Item2, Item1, }
// impl Dim2ListTableID{
// 	pub(crate ) fn from_str(id : &str) -> Option<Self>{
// 		match id{
// 			"item3" => Some(Self::Item3),
// 			"item2" => Some(Self::Item2),
// 			"item1" => Some(Self::Item1),
// 			_ =>{ None }
// 		}
// 	}
// 	pub(crate ) fn from_num(id : usize) -> Self{
// 		match id{
// 			0 => Self::Item3,
// 			1 => Self::Item2,
// 			2 => Self::Item1,
// 			_ => panic!("invalid ID num {} Dim2ListTableID", id),
// 		}
// 	}
// 	pub(crate ) fn len() -> usize{ 3 }
// 	pub(crate ) fn to_num(&self) -> usize{
// 		match self{
// 			Dim2ListTableID::Item3 => 0,
// 			Dim2ListTableID::Item2 => 1,
// 			Dim2ListTableID::Item1 => 2,
// 		}
// 	}
// 	pub(crate ) fn metadata() -> &'static [&'static str]{
// 		&["item3", "item2", "item1", ]
// 	}
// 	pub(crate ) fn to_str(&self) -> &'static str{
// 		Self::metadata()[self.to_num()]
// 	}
// }
// #[derive(Debug, PartialEq)]
// pub(crate ) struct Dim2ListCItem {
// 	ptr : CItemPtr,
// }
// impl From<CItemPtr> for Dim2ListCItem {
// 	fn from(ptr : CItemPtr) -> Self { Self{ ptr } }
// }
// impl Dim2ListCItem {
// 	pub(crate ) fn data_mem(&self) -> i64{
// 		let qv = citem::get_int(self.ptr, "dataMem").unwrap();
// 		qv.into_value().unwrap()
// 	}
//
// }
//
// #[derive(Debug, PartialEq)]
// pub(crate ) struct UnkoListTable {
// 	ptr : TablePtr,
// }
// impl UnkoListTable {
// 	pub(crate ) fn new(ptr : TablePtr) -> UnkoListTable{ UnkoListTable{ ptr } }
// 	pub(crate ) fn first(&self) -> UnkoListCItem {
// 		let ptr = table::get_value(self.ptr, "first").unwrap();
// 		UnkoListCItem::from(ptr)
// 	}
// 	pub(crate ) fn second(&self) -> UnkoListCItem {
// 		let ptr = table::get_value(self.ptr, "second").unwrap();
// 		UnkoListCItem::from(ptr)
// 	}
// 	pub(crate ) fn from_id(&self, id : UnkoListTableID) -> UnkoListCItem{
// 		match id{
// 			UnkoListTableID::First => self.first(),
// 			UnkoListTableID::Second => self.second(),
// 		}
// 	}
// }
// #[repr(u64)] pub(crate ) enum UnkoListTableID{ First, Second, }
// impl UnkoListTableID{
// 	pub(crate ) fn from_str(id : &str) -> Option<Self>{
// 		match id{
// 			"first" => Some(Self::First),
// 			"second" => Some(Self::Second),
// 			_ =>{ None }
// 		}
// 	}
// 	pub(crate ) fn from_num(id : usize) -> Self{
// 		match id{
// 			0 => Self::First,
// 			1 => Self::Second,
// 			_ => panic!("invalid ID num {} UnkoListTableID", id),
// 		}
// 	}
// 	pub(crate ) fn len() -> usize{ 2 }
// 	pub(crate ) fn to_num(&self) -> usize{
// 		match self{
// 			UnkoListTableID::First => 0,
// 			UnkoListTableID::Second => 1,
// 		}
// 	}
// 	pub(crate ) fn metadata() -> &'static [&'static str]{
// 		&["first", "second", ]
// 	}
// 	pub(crate ) fn to_str(&self) -> &'static str{
// 		Self::metadata()[self.to_num()]
// 	}
// }
// #[derive(Debug, PartialEq)]
// pub(crate ) struct UnkoListCItem {
// 	ptr : CItemPtr,
// }
// impl From<CItemPtr> for UnkoListCItem {
// 	fn from(ptr : CItemPtr) -> Self { Self{ ptr } }
// }
// impl UnkoListCItem {
// 	pub(crate ) fn hoge_int(&self) -> i64{
// 		let qv = citem::get_int(self.ptr, "hogeInt").unwrap();
// 		qv.into_value().unwrap()
// 	}
// 	pub(crate ) fn hoge_string(&self) -> String{
// 		let qv = citem::get_str(self.ptr, "hogeString").unwrap();
// 		qv.into_value().unwrap()
// 	}
//
// }
//
// #[derive(Debug, PartialEq)]
// pub(crate ) struct HegoListTable {
// 	ptr : TablePtr,
// }
// impl HegoListTable {
// 	pub(crate ) fn new(ptr : TablePtr) -> HegoListTable{ HegoListTable{ ptr } }
// 	pub(crate ) fn nantoka_old(&self) -> HegoListCItem {
// 		let ptr = table::get_value(self.ptr, "nantoka").unwrap();
// 		HegoListCItem::from(ptr)
// 	}
// 	pub(crate ) fn hegohego(&self) -> HegoListCItem {
// 		let ptr = table::get_value(self.ptr, "hegohego").unwrap();
// 		HegoListCItem::from(ptr)
// 	}
// 	pub(crate ) fn from_id(&self, id : HegoListTableID) -> HegoListCItem{
// 		match id{
// 			HegoListTableID::NantokaOld => self.nantoka_old(),
// 			HegoListTableID::Hegohego => self.hegohego(),
// 		}
// 	}
// }
// #[repr(u64)] pub(crate ) enum HegoListTableID{ NantokaOld, Hegohego, }
// impl HegoListTableID{
// 	pub(crate ) fn from_str(id : &str) -> Option<Self>{
// 		match id{
// 			"nantoka" => Some(Self::NantokaOld),
// 			"hegohego" => Some(Self::Hegohego),
// 			_ =>{ None }
// 		}
// 	}
// 	pub(crate ) fn from_num(id : usize) -> Self{
// 		match id{
// 			0 => Self::NantokaOld,
// 			1 => Self::Hegohego,
// 			_ => panic!("invalid ID num {} HegoListTableID", id),
// 		}
// 	}
// 	pub(crate ) fn len() -> usize{ 2 }
// 	pub(crate ) fn to_num(&self) -> usize{
// 		match self{
// 			HegoListTableID::NantokaOld => 0,
// 			HegoListTableID::Hegohego => 1,
// 		}
// 	}
// 	pub(crate ) fn metadata() -> &'static [&'static str]{
// 		&["nantoka", "hegohego", ]
// 	}
// 	pub(crate ) fn to_str(&self) -> &'static str{
// 		Self::metadata()[self.to_num()]
// 	}
// }
// #[derive(Debug, PartialEq)]
// pub(crate ) struct HegoListCItem {
// 	ptr : CItemPtr,
// }
// impl From<CItemPtr> for HegoListCItem {
// 	fn from(ptr : CItemPtr) -> Self { Self{ ptr } }
// }
// impl HegoListCItem {
// 	pub(crate ) fn mem(&self) -> String{
// 		let qv = citem::get_str(self.ptr, "mem").unwrap();
// 		qv.into_value().unwrap()
// 	}
//
// }
//
// #[derive(Debug, PartialEq)]
// pub(crate ) struct Mut2MItem {
// 	ptr : MItemPtr,
// }
// impl From<MItemPtr> for Mut2MItem {
// 	fn from(ptr : MItemPtr) -> Self {
// 		Self{ ptr }
// 	}
// }
// impl Mut2MItem {
// 	pub(crate ) fn mem(&self) -> String{
// 		let qv = mitem::get_str(self.ptr, "mem").unwrap();
// 		qv.into_value().unwrap()
// 	}
// 	pub(crate ) fn set_mem(&mut self, mem : String){
// 		mitem::set_str(self.ptr, "mem", Qv::Val(mem));
// 	}
// 	pub(crate ) fn get_enum(&self) -> Mut2Enum{
// 		let (list_name, _) = mitem::get_enum(self.ptr).unwrap();
// 		let p = if let Qv::Val(p) = mitem::get_ref(self.ptr, &list_name).unwrap(){ p } else { unreachable!() };
// 		Mut2Enum::new(&list_name,p)
// 	}
// 	pub(crate ) fn get_enum_ids(&self) -> (String,String){
// 		mitem::get_enum(self.ptr).unwrap()
// 	}
// 	pub(crate ) fn set_enum(&self, kind : Mut2Kind){
// 		let (list_name, id) = kind.id();
// 		mitem::set_enum(self.ptr, list_name, id);
// 	}
// }
// pub(crate ) enum Mut2Enum{ HogeList(HogeListCItem), HegoList(HegoListCItem), HugaList(HugaListCItem), }
// impl Mut2Enum{
// 	pub(crate ) fn new(list_name : &str, ptr : CItemPtr) -> Mut2Enum{
// 		match list_name{
// 			"hogeList" => Mut2Enum::HogeList(HogeListCItem::from(ptr)),
// 			"hegoList" => Mut2Enum::HegoList(HegoListCItem::from(ptr)),
// 			"hugaList" => Mut2Enum::HugaList(HugaListCItem::from(ptr)),
// 			_ => panic!("Mut2Enum there's no enum type named {}", &list_name),
// 		}
// 	}
// }
// pub(crate ) enum Mut2Kind{ HogeList(HogeListTableID), HegoList(HegoListTableID), HugaList(HugaListTableID), }
// impl Mut2Kind{
// 	pub(crate ) fn id(&self) -> (&'static str, &'static str){
// 		match self{
// 			Mut2Kind::HogeList(v) => ("hogeList", v.to_str()),
// 			Mut2Kind::HegoList(v) => ("hegoList", v.to_str()),
// 			Mut2Kind::HugaList(v) => ("hugaList", v.to_str()),
// 		}
// 	}
// }
//
// #[derive(Debug, PartialEq)]
// pub(crate ) struct EnumListCItem {
// 	ptr : CItemPtr,
// }
// impl From<CItemPtr> for EnumListCItem {
// 	fn from(ptr : CItemPtr) -> Self { Self{ ptr } }
// }
// impl EnumListCItem {
// 	pub(crate ) fn mem(&self) -> String{
// 		let qv = citem::get_str(self.ptr, "mem").unwrap();
// 		qv.into_value().unwrap()
// 	}
// 	pub(crate ) fn get_enum(&self) -> EnumListEnum{
// 		let (list_name, _) = citem::get_enum(self.ptr).unwrap();
// 		let p = if let Qv::Val(p) = citem::get_ref(self.ptr, &list_name).unwrap(){ p } else { unreachable!() };
// 		EnumListEnum::new(&list_name,p)
// 	}
// 	pub(crate ) fn get_enum_ids(&self) -> (String,String){
// 		citem::get_enum(self.ptr).unwrap()
// 	}
// }
// pub(crate ) enum EnumListEnum{ HogeList(HogeListCItem), HegoList(HegoListCItem), HugaList(HugaListCItem), }
// impl EnumListEnum{
// 	pub(crate ) fn new(list_name : &str, ptr : CItemPtr) -> EnumListEnum{
// 		match list_name{
// 			"hogeList" => EnumListEnum::HogeList(HogeListCItem::from(ptr)),
// 			"hegoList" => EnumListEnum::HegoList(HegoListCItem::from(ptr)),
// 			"hugaList" => EnumListEnum::HugaList(HugaListCItem::from(ptr)),
// 			_ => panic!("EnumListEnum there's no enum type named {}", &list_name),
// 		}
// 	}
// }
//
