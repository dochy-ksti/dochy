use dochy::core::json_dir_to_root;
use dochy::intf::generate_interface;

#[test]
fn clilst_new_generate() {
    match json_dir_to_root("src/b3_1_clist_and_mlist/jsons/clist_new", true) {
        Ok(mut a) => {
            let ans = generate_interface(&mut a);
            std::fs::write(
                "src/b3_1_clist_and_mlist/clist_new_accessor.rs",
                &ans.to_string(),
            ).unwrap();
        }
        Err(_e) => {
            assert!(false);
        }
    }
}