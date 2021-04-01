use dochy_core::json_dir_to_rust;
use dochy_intf::generate_interface;

#[test]
fn poly_generate() {
    match json_dir_to_rust("src/sample_test/sample_code_json/poly", true) {
        Ok(mut a) => {
            let ans = generate_interface(&mut a);
            std::fs::write(
                "src/sample_test/sample_code/poly_accessor.rs",
                &ans.to_string(),
            ).unwrap();
        }
        Err(e) => {
            dbg!(e);
            assert!(false);
        }
    }
}