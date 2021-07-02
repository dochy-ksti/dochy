use dochy::core::json_dir_to_root;
use dochy::intf::generate_interface;

#[test]
fn rpg2_generate() {
    match json_dir_to_root("src/sample_test/sample_code_json/rpg2", true) {
        Ok(mut a) => {
            let ans = generate_interface(&mut a);
            std::fs::write(
                "src/sample_test/sample_code/rpg2_accessor.rs",
                &ans.to_string(),
            ).unwrap();
        }
        Err(e) => {
            assert!(false);
        }
    }
}