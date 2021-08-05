use dochy::core::json_dir_to_root;
use dochy::intf::generate_interface;

#[test]
fn mlilst_mut_generate() {
    match json_dir_to_root("src/mlist_mut/json/", true) {
        Ok(mut a) => {
            let ans = generate_interface(&mut a);
            std::fs::write(
                "src/mlist_mut/mlist_mut_accessor.rs",
                &ans.to_string(),
            ).unwrap();
        }
        Err(_e) => {
            assert!(false);
        }
    }
}