

#[cfg(test)]
mod tests {
    use crate::testing::diff::util::get_root_obj::get_root_obj;
    use nougami_core::intf::null_or::{NullOr, UndefOr};
    use nougami_core::structs::Qv;
    use crate::testing::diff::generated_test_ref::test::{RootIntf, Refed1TableID, Refed2TableID, Refed3TableID, Refed4TableID};

    #[test]
    fn test_diff2() -> Result<(), String>{
        let json_dir_path = "src/testing/diff/diff_ref/";
        let root_obj = get_root_obj(json_dir_path)?;

        let mut intf = RootIntf::new(root_obj);

        let f = intf.list1().first().unwrap();
        f.set_ref_refed1(Refed1TableID::A2);
        f.set_ref_refed2(NullOr::Val(Refed2TableID::B2));
        f.set_ref_refed3(UndefOr::Val(Refed3TableID::C2));
        f.set_ref_refed4(Qv::Val(Refed4TableID::D2));


        let moto = get_root_obj(json_dir_path)?;

        let diff = nougami_diff::get_diff(&moto, unsafe{ intf.root_obj_ref() }).or_else(|e| Err(e.message))?;
        let applied = nougami_diff::apply_diff(moto, diff).or_else(|e| Err(e.message))?;
        let mut intf = RootIntf::new(applied);
        let f = intf.list1().first().unwrap();


        assert_eq!(f.ref_id_refed1(), "a2".to_string());
        assert_eq!(f.ref_id_refed2(), NullOr::Val("b2".to_string()));
        assert_eq!(f.ref_id_refed3(), UndefOr::Val("c2".to_string()));
        assert_eq!(f.ref_id_refed4(), Qv::Val("d2".to_string()));

        let f = intf.list1().last().unwrap();
        f.set_ref_refed1(Refed1TableID::A1);
        f.set_ref_refed2(NullOr::Val(Refed2TableID::B1));
        f.set_ref_refed3(UndefOr::Val(Refed3TableID::C1));
        f.set_ref_refed4(Qv::Val(Refed4TableID::D1));

        let moto = get_root_obj(json_dir_path)?;

        let diff = nougami_diff::get_diff(&moto, unsafe{ intf.root_obj_ref() }).or_else(|e| Err(e.message))?;
        let applied = nougami_diff::apply_diff(moto, diff).or_else(|e| Err(e.message))?;
        let mut intf = RootIntf::new(applied);
        let f = intf.list1().last().unwrap();

        assert_eq!(f.ref_id_refed1(), "a1".to_string());
        assert_eq!(f.ref_id_refed2(), NullOr::Val("b1".to_string()));
        assert_eq!(f.ref_id_refed3(), UndefOr::Val("c1".to_string()));
        assert_eq!(f.ref_id_refed4(), Qv::Val("d1".to_string()));

        let f = intf.list2().first().unwrap();
        f.set_ref_refed1(Refed1TableID::A2);
        f.set_ref_refed2(NullOr::Val(Refed2TableID::B2));
        f.set_ref_refed3(UndefOr::Val(Refed3TableID::C2));
        f.set_ref_refed4(Qv::Val(Refed4TableID::D2));


        let moto = get_root_obj(json_dir_path)?;

        let diff = nougami_diff::get_diff(&moto, unsafe{ intf.root_obj_ref() }).or_else(|e| Err(e.message))?;
        let applied = nougami_diff::apply_diff(moto, diff).or_else(|e| Err(e.message))?;
        let mut intf = RootIntf::new(applied);
        let f = intf.list2().first().unwrap();

        assert_eq!(f.ref_id_refed1(), "a2".to_string());
        assert_eq!(f.ref_id_refed2(), NullOr::Val("b2".to_string()));
        assert_eq!(f.ref_id_refed3(), UndefOr::Val("c2".to_string()));
        assert_eq!(f.ref_id_refed4(), Qv::Val("d2".to_string()));

        let f = intf.list2().last().unwrap();
        //f.set_ref_refed1(Refed1TableID::A1);
        f.set_ref_refed2(NullOr::Null);
        f.set_ref_refed3(UndefOr::Undefined);
        f.set_ref_refed4(Qv::Null);

        let moto = get_root_obj(json_dir_path)?;

        let diff = nougami_diff::get_diff(&moto, unsafe{ intf.root_obj_ref() }).or_else(|e| Err(e.message))?;
        let applied = nougami_diff::apply_diff(moto, diff).or_else(|e| Err(e.message))?;
        let mut intf = RootIntf::new(applied);
        let f = intf.list2().last().unwrap();

        //assert_eq!(f.ref_id_refed1(), "a1".to_string());
        assert_eq!(f.ref_id_refed2(), NullOr::Null);
        assert_eq!(f.ref_id_refed3(), UndefOr::Undefined);
        assert_eq!(f.ref_id_refed4(), Qv::Null);


        let f = intf.list3().first().unwrap();
        //f.set_ref_refed1(Refed1TableID::A2);
        f.set_ref_refed2(NullOr::Null);
        f.set_ref_refed3(UndefOr::Undefined);
        f.set_ref_refed4(Qv::Undefined);


        let moto = get_root_obj(json_dir_path)?;

        let diff = nougami_diff::get_diff(&moto, unsafe{ intf.root_obj_ref() }).or_else(|e| Err(e.message))?;
        let applied = nougami_diff::apply_diff(moto, diff).or_else(|e| Err(e.message))?;
        let mut intf = RootIntf::new(applied);
        let f = intf.list3().first().unwrap();

        //assert_eq!(f.ref_id_refed1(), "a2".to_string());
        assert_eq!(f.ref_id_refed2(), NullOr::Null);
        assert_eq!(f.ref_id_refed3(), UndefOr::Undefined);
        assert_eq!(f.ref_id_refed4(), Qv::Undefined);

        let f = intf.list3().last().unwrap();
        //f.set_ref_refed1(Refed1TableID::A1);
        f.set_ref_refed2(NullOr::Null);
        f.set_ref_refed3(UndefOr::Undefined);
        f.set_ref_refed4(Qv::Undefined);

        let moto = get_root_obj(json_dir_path)?;

        let diff = nougami_diff::get_diff(&moto, unsafe{ intf.root_obj_ref() }).or_else(|e| Err(e.message))?;
        let applied = nougami_diff::apply_diff(moto, diff).or_else(|e| Err(e.message))?;
        let mut intf = RootIntf::new(applied);
        let f = intf.list3().last().unwrap();

        //assert_eq!(f.ref_id_refed1(), "a1".to_string());
        assert_eq!(f.ref_id_refed2(), NullOr::Null);
        assert_eq!(f.ref_id_refed3(), UndefOr::Undefined);
        assert_eq!(f.ref_id_refed4(), Qv::Undefined);
        Ok(())
    }


}