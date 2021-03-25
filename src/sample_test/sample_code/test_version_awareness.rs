use dochy_core::{json_dir_to_rust, adjust_versions};
use dochy_intf::generate_interface;
use crate::sample_test::sample_code::version_awareness_accessor::RootIntf;
use crate::sample_test::sample_code::version_awareness_accessor_wrapper::VeraAccessorWrapper;
use crate::core::CoreResult;
use dochy_core::intf::RootObjectPtr;

#[test]
fn generate_version_awareness() {
    match json_dir_to_rust("src/sample_test/sample_code_json/version_aware/new", true) {
        Ok(mut a) => {
            let ans = generate_interface(&mut a);
            std::fs::write(
                "src/sample_test/sample_code/version_awareness_accessor.rs",
                &ans.to_string(),
            )
            .unwrap();
        }
        Err(e) => {
            dbg!(e);
            assert!(false);
        }
    }
}

#[test]
fn test_version_awareness() -> CoreResult<()>{
    let old = json_dir_to_rust("src/sample_test/sample_code_json/version_aware/old", true)?;
    let new = json_dir_to_rust("src/sample_test/sample_code_json/version_aware/new", true)?;
    let r = adjust_versions(new, old, true)?;
    let r =RootIntf::new(r);
    let r = VeraAccessorWrapper::new(r);
    assert_eq!(r.new_value(), 100);

    let mut old = json_dir_to_rust("src/sample_test/sample_code_json/version_aware/old", true)?;
    crate::core::intf::root::set_int(RootObjectPtr::new(&mut old), "oldValue", crate::core::structs::Qv::Val(2));
    let new = json_dir_to_rust("src/sample_test/sample_code_json/version_aware/new", true)?;
    let r = adjust_versions(new, old, true)?;
    let r =RootIntf::new(r);
    let r = VeraAccessorWrapper::new(r);
    assert_eq!(r.new_value(), 20);
    Ok(())
}
