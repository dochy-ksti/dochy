use dochy_core::intf::{RootObjectPtr, MListPtr, MItemPtr};
use dochy_core::intf::{mitem};
use dochy_core::structs::Qv;
use dochy_core::rust_to_json_new_default;
use dochy_diff::apply_diff;
use crate::sample_test::error::DpResult;


//#[test]
fn sample_employee_minimum() -> DpResult<()>{
    let ini_path= "sample_code_json/sample_employee_dochy_minimum";
    let mut r = dochy_core::json_dir_to_rust(ini_path, false)?;

    let rp = RootObjectPtr::new(&mut r);

    let mut ml : MListPtr<MItemPtr> = dochy_core::intf::root::get_mlist(
        rp, "@Employees")?;
    let item = ml.insert();
    mitem::set_str(item, "userId", Qv::Val("krish".to_string()));
    mitem::set_str(item, "jobTitle", Qv::Val("Developer".to_string()));
    mitem::set_str(item, "firstName", Qv::Val("Krish".to_string()));
    mitem::set_str(item, "lastName", Qv::Val("Lee".to_string()));
    mitem::set_str(item, "employeeCode", Qv::Val("E1".to_string()));
    mitem::set_str(item, "region", Qv::Val("CA".to_string()));
    mitem::set_str(item, "phoneNumber", Qv::Val("123456".to_string()));
    mitem::set_str(item, "emailAddress",
                   Qv::Val("krish.lee@learningcontainer.com".to_string()));
    let item = ml.insert();
    mitem::set_str(item, "userId", Qv::Val("devid".to_string()));
    mitem::set_str(item, "jobTitle", Qv::Val("Developer".to_string()));
    mitem::set_str(item, "firstName", Qv::Val("Devid".to_string()));
    mitem::set_str(item, "lastName", Qv::Val("Rome".to_string()));
    mitem::set_str(item, "employeeCode", Qv::Val("E2".to_string()));
    mitem::set_str(item, "region", Qv::Val("CA".to_string()));
    mitem::set_str(item, "phoneNumber", Qv::Val("1111111".to_string()));
    mitem::set_str(item, "emailAddress",
                   Qv::Val("devid.rome@learningcontainer.com".to_string()));

    let item = ml.insert();
    mitem::set_str(item, "userId", Qv::Val("tin".to_string()));
    mitem::set_str(item, "jobTitle", Qv::Val("Program Directory".to_string()));
    mitem::set_str(item, "firstName", Qv::Val("tin".to_string()));
    mitem::set_str(item, "lastName", Qv::Val("jonson".to_string()));
    mitem::set_str(item, "employeeCode", Qv::Val("E3".to_string()));
    mitem::set_str(item, "region", Qv::Val("CA".to_string()));
    mitem::set_str(item, "phoneNumber", Qv::Val("2222222".to_string()));
    mitem::set_str(item, "emailAddress",
                   Qv::Val("tin.jonson@learningcontainer.com".to_string()));
    //let hoge = rust_to_json_new_default(&r)?;
    //println!("{}", hoge.to_string_pretty());

    let mut from = dochy_core::json_dir_to_rust(
        ini_path, false)?;
    let vec = dochy_diff::get_diff(&from, &r)?;
    println!("{}", vec.len());
    apply_diff(&mut from, &mut vec.as_slice())?;

    let from = rust_to_json_new_default(&r)?;
    println!("{}", from.to_string_pretty());
    Ok(())

}