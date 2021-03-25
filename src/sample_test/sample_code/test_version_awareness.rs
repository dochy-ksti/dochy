use dochy_core::json_dir_to_rust;
use dochy_intf::generate_interface;
use crate::sample_test::sample_code::version_awareness_accessor::RootIntf;

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
fn test_version_awareness() {
    let a = match json_dir_to_rust("src/sample_test/sample_code_json/version_aware/new", true) {
        Ok(mut a) => a,
        Err(e) => {
            dbg!(e);
            assert!(false);
            return;
        }
    };
    let mut r =RootIntf::new(a);
}
