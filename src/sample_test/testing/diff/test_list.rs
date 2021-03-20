

#[cfg(test)]
mod tests {
    use dochy_core::structs::{RootObject};
    use crate::sample_test::testing::diff::generated_test_list::test::{RootIntf, Refed1TableID};
    use crate::sample_test::testing::diff::util::get_root_obj::get_root_obj;
    use crate::sample_test::error::DpResult;

    fn apply(current : &RootObject, path : &str) -> DpResult<RootIntf>{
        let mut moto = get_root_obj(path)?;

        let diff = dochy_diff::get_diff(&moto, current)?;
        dochy_diff::apply_diff(&mut moto, &mut diff.as_slice())?;
        Ok(RootIntf::new(moto))
    }

    #[test]
    fn test_modify() -> DpResult<()>{
        let json_dir_path = "src/sample_test/testing/diff/diff_list/";
        let root_obj = get_root_obj(json_dir_path)?;

        let mut intf = RootIntf::new(root_obj);

        intf.list().first().unwrap().set_mem(2);

        let mut intf = apply(unsafe{intf.root_obj_ref()}, json_dir_path)?;

        assert_eq!(intf.list().first().unwrap().mem(), 2);

        Ok(())
    }

    #[test]
    fn test_add_add_remove() -> DpResult<()>{
        let json_dir_path = "src/sample_test/testing/diff/diff_list/";
        let root_obj = get_root_obj(json_dir_path)?;

        let mut intf = RootIntf::new(root_obj);

        intf.list().insert_last();
        let hoge = intf.list().insert_last();
        hoge.set_ref_refed1(Refed1TableID::A2);
        intf.list().remove_first();

        let mut intf = apply(unsafe{intf.root_obj_ref()}, json_dir_path)?;
        let mut list = intf.list();
        assert_eq!(list.len(), 2);
        assert_eq!(list.first_id().unwrap(), 1);
        assert_eq!(list.last_id().unwrap(), 2);
        assert_eq!(list.last().unwrap().ref_id_refed1(), "a2");

        Ok(())
    }

    #[test]
    fn test_remove_add_add_add_remove_user(){
        match test_remove_add_add_add_remove(){
            Ok(_)=>{},
            Err(e) => println!("{}", e),
        }
    }

    fn test_remove_add_add_add_remove() -> DpResult<()>{
        let json_dir_path = "src/sample_test/testing/diff/diff_list/";
        let root_obj = get_root_obj(json_dir_path)?;

        let mut intf = RootIntf::new(root_obj);

        let mut list = intf.list();
        list.remove_first();
        list.insert_last();
        list.insert_first();
        list.insert_last();
        let id = list.last_id().unwrap();
        list.insert_last();
        list.remove(id);

        let mut iter = list.iter();
        let (id, _item) = iter.next().unwrap();
        assert_eq!(id, 2);
        let (id, _item) = iter.next().unwrap();
        assert_eq!(id, 1);
        let (id, _item) = iter.next().unwrap();
        assert_eq!(id, 4);

        let mut intf = apply(unsafe{intf.root_obj_ref()}, json_dir_path)?;
        let mut list = intf.list();
        assert_eq!(list.len(), 3);
        let mut iter = list.iter();
        let (id, _item) = iter.next().unwrap();
        assert_eq!(id, 2);
        let (id, _item) = iter.next().unwrap();
        assert_eq!(id, 1);
        let (id, _item) = iter.next().unwrap();
        assert_eq!(id, 4);

        Ok(())
    }


    #[test]
    fn test_in_add_add_add_add_remove_user(){
        match test_remove_add_add_add_remove(){
            Ok(_)=>{},
            Err(e) => println!("{}", e),
        }
    }

    fn test_in_add_add_add_add_remove() -> DpResult<()>{
        let json_dir_path = "src/sample_test/testing/diff/diff_list/";
        let root_obj = get_root_obj(json_dir_path)?;

        let mut intf = RootIntf::new(root_obj);

        let mut list = intf.list();
        list.insert_last();

        let mut f = list.first().unwrap().in_list();
        let mut l = list.last().unwrap().in_list();
        l.remove_first();

        f.insert_first();
        f.insert_first();
        f.insert_last();
        f.insert_last();
        f.remove_last();
        let mut item = f.last().unwrap();
        item.set_in_mem(4);

        let mut intf = apply(unsafe{intf.root_obj_ref()}, json_dir_path)?;
        let mut list = intf.list();
        assert_eq!(list.len(), 2);

        let mut iter = list.first().unwrap().in_list().iter();
        let (id, _item) = iter.next().unwrap();
        assert_eq!(id, 2);
        let (id, _item) = iter.next().unwrap();
        assert_eq!(id, 1);
        let (id, _item) = iter.next().unwrap();
        assert_eq!(id, 0);
        let (id, _item) = iter.next().unwrap();
        assert_eq!(id, 4);

        let mut list = list.last().unwrap().in_list();

        assert_eq!(list.len(), 0);

        for _item in list.iter(){
            println!("nai ");
        }
        println!("ok");
        Ok(())
    }





}