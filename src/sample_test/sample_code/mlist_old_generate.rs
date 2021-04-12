use dochy_core::json_dir_to_root;
use dochy_intf::generate_interface;

#[test]
fn mlilst_old_generate() {
    match json_dir_to_root("src/sample_test/sample_code_json/mlist_old", true) {
        Ok(mut a) => {
            let ans = generate_interface(&mut a);
            std::fs::write(
                "src/sample_test/sample_code/mlist_old_accessor.rs",
                &ans.to_string(),
            ).unwrap();
        }
        Err(e) => {
            dbg!(e);
            assert!(false);
        }
    }
}