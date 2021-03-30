use dochy_core::intf::{RootObjectPtr, MListPtr, MItemPtr};
use dochy_core::intf::{mitem};
use dochy_core::structs::Qv;
use dochy_core::rust_to_json_new_default;
use dochy_diff::apply_diff;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::io::prelude::*;
use crate::error::DpResult;


//#[test]
fn users_omitted() -> DpResult<()>{
    let ini_path= "src/sample_code_json/users/users_initial";
    let mut r = dochy_core::json_dir_to_rust(ini_path, false)?;

    let rp = RootObjectPtr::new(&mut r);

    let mut ml : MListPtr<MItemPtr> = dochy_core::intf::root::get_mlist(
        rp, "users")?;
    let item = ml.insert();
    mitem::set_int(item, "id", Qv::Val(1));
    mitem::set_str(item, "name", Qv::Val("Holt Gianuzzi".to_string()));
    mitem::set_str(item, "created", Qv::Val("2/24/2021".to_string()));

    let item = ml.insert();
    mitem::set_int(item, "id", Qv::Val(2));
    mitem::set_str(item, "name", Qv::Val("Sansone Gerard".to_string()));
    mitem::set_str(item, "created", Qv::Val("5/28/2020".to_string()));
    mitem::set_bool(item, "is_active", Qv::Val(true));

    let item = ml.insert();
    mitem::set_int(item, "id", Qv::Val(3));
    mitem::set_str(item, "name", Qv::Val("Max Mangan".to_string()));
    mitem::set_str(item, "created", Qv::Val("3/14/2020".to_string()));

    let item = ml.insert();
    mitem::set_int(item, "id", Qv::Val(4));
    mitem::set_str(item, "name", Qv::Val("Brice Bartosch".to_string()));
    mitem::set_str(item, "created", Qv::Val("11/24/2020".to_string()));

    let item = ml.insert();
    mitem::set_int(item, "id", Qv::Val(5));
    mitem::set_str(item, "name", Qv::Val("Lauretta Tyt".to_string()));
    mitem::set_str(item, "created", Qv::Val("4/26/2020".to_string()));
    mitem::set_bool(item, "is_active", Qv::Val(true));

    let mut cloned = r.clone();

    let item = ml.insert();
    mitem::set_int(item, "id", Qv::Val(6));
    mitem::set_str(item, "name", Qv::Val("Pete Blincow".to_string()));
    mitem::set_str(item, "created", Qv::Val("6/11/2020".to_string()));
    mitem::set_bool(item, "is_active", Qv::Val(true));

    //let hoge = rust_to_json_new_default(&r)?;
    //println!("{}", hoge.to_string_pretty());

    let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
    let js = std::fs::read("src/sample_code_json/users/users_added.json5")?;
    dbg!(js.len());
    e.write_all(&js)?;
    let compressed_bytes = e.finish()?;
    dbg!(compressed_bytes.len());

    // let mut from = dochy_core::json_dir_to_rust(
    //     ini_path, false)?;
    let vec = dochy_diff::get_diff(&cloned, &r)?;
    println!("{}", vec.len());
    apply_diff(&mut cloned, &mut vec.as_slice())?;

    let from = rust_to_json_new_default(&r)?;
    println!("{}", from.to_string_pretty());
    Ok(())

}