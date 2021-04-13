use dochy::core::json_dir_to_root;
use dochy::intf::generate_interface;

#[test]
fn ref1_generate() {
    match json_dir_to_root("src/sample_test/sample_code_json/ref1", true) {
        Ok(mut a) => {
            let ans = generate_interface(&mut a);
            std::fs::write(
                "src/sample_test/sample_code/ref1_accessor.rs",
                &ans.to_string(),
            ).unwrap();
        }
        Err(e) => {
            dbg!(e);
            assert!(false);
        }
    }
}