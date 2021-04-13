use dochy::core::json_dir_to_root;
use dochy::intf::generate_interface;

#[test]
fn mlilst_new_generate() {
    match json_dir_to_root("src/sample_test/sample_code_json/mlist_new", true) {
        Ok(mut a) => {
            let ans = generate_interface(&mut a);
            std::fs::write(
                "src/sample_test/sample_code/mlist_new_accessor.rs",
                &ans.to_string(),
            ).unwrap();
        }
        Err(e) => {
            dbg!(e);
            assert!(false);
        }
    }
}