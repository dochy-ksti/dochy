use dochy::core::intf::{RootObjectPtr, MListPtr, MItemPtr};
use dochy::core::intf::{mitem};
use dochy::core::structs::Qv;
use dochy::core::root_to_json_new_default;
use dochy::diff::apply_diff;
use dochy::error::DpResult;


//#[test]
fn whats_statically() -> DpResult<()>{
    let mut r = dochy::core::json_dir_to_root(
        "sample_code_json/whats_statically_dochy_first", false)?;

    let rp = RootObjectPtr::new(&mut r);

    let mut ml : MListPtr<MItemPtr> = dochy::core::intf::root::get_mlist(rp, "list")?;
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

    let mut from = dochy::core::json_dir_to_root(
        "sample_code_json/whats_statically_dochy_first", false)?;
    let vec = dochy::diff::get_diff(&from, &r)?;
    println!("{}", vec.len());
    apply_diff(&mut from, &mut vec.as_slice())?;

    let from = root_to_json_new_default(&r)?;
    println!("{}", from.to_string_pretty());
    Ok(())

}