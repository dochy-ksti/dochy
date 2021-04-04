use dochy_core::json_dir_to_rust;
use dochy_intf::generate_interface;

#[test]
fn ref2_generate() {
    match json_dir_to_rust("src/sample_test/sample_code_json/ref2", true) {
        Ok(mut a) => {
            let ans = generate_interface(&mut a);
            std::fs::write(
                "src/sample_test/sample_code/ref2_accessor.rs",
                &ans.to_string(),
            ).unwrap();
        }
        Err(e) => {
            dbg!(e);
            assert!(false);
        }
    }
}