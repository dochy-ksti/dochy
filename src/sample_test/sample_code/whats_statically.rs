use dochy_core::intf::{RootObjectPtr, MListPtr, MItemPtr};
use dochy_core::intf::{mitem};
use dochy_core::structs::Qv;
use dochy_core::rust_to_json_new_default;
use dochy_diff::apply_diff;
use crate::sample_test::error::DpResult;


//#[test]
fn whats_statically() -> DpResult<()>{
    let mut r = dochy_core::json_dir_to_rust(
        "sample_code_json/whats_statically_dochy_first", false)?;

    let rp = RootObjectPtr::new(&mut r);

    let mut ml : MListPtr<MItemPtr> = dochy_core::intf::root::get_mlist(rp, "list")?;
    let item = ml.get_item(0)?;
    mitem::set_int(item, "age", Qv::Val(101));
    mitem::set_str(item, "name", Qv::Val("dochy101".to_string()));
    let item = ml.get_item(1)?;
    mitem::set_int(item, "age", Qv::Val(102));
    mitem::set_str(item, "name", Qv::Val("dochy102".to_string()));
    // let item = ml.insert();
    // mitem::set_int(item, "age", Qv::Val(3));
    // mitem::set_str(item, "name", Qv::Val("dochy3".to_string()));
    // let item = ml.insert();
    // mitem::set_int(item, "age", Qv::Val(4));
    // mitem::set_str(item, "name", Qv::Val("dochy4".to_string()));
    // let item = ml.insert();
    // mitem::set_int(item, "age", Qv::Val(5));
    // mitem::set_str(item, "name", Qv::Val("dochy5".to_string()));

    //let hoge = rust_to_json_new_default(&r)?;
    //println!("{}", hoge.to_string_pretty());

    let mut from = dochy_core::json_dir_to_rust(
        "sample_code_json/whats_statically_dochy_first", false)?;
    let vec = dochy_diff::get_diff(&from, &r)?;
    println!("{}", vec.len());
    apply_diff(&mut from, &mut vec.as_slice())?;

    let from = rust_to_json_new_default(&r)?;
    println!("{}", from.to_string_pretty());
    Ok(())

}