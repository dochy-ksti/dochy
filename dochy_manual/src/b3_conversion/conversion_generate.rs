use dochy::core::{json_dir_to_root};
use dochy::intf::generate_interface;

#[test]
fn generate_old() {
    match json_dir_to_root("src/b3_conversion/jsons/old", true) {
        Ok(mut a) => {
            let ans = generate_interface(&mut a);
            std::fs::write(
                "src/b3_conversion/old_accessor.rs",
                &ans.to_string(),
            ).unwrap();
        }
        Err(_e) => {
            assert!(false);
        }
    }
}
#[test]
fn generate_new() {
    match json_dir_to_root("src/b3_conversion/jsons/new", true) {
        Ok(mut a) => {
            let ans = generate_interface(&mut a);
            std::fs::write(
                "src/b3_conversion/new_accessor.rs",
                &ans.to_string(),
            ).unwrap();
        }
        Err(_e) => {
            assert!(false);
        }
    }
}
#[test]
fn generate_new2() {
    match json_dir_to_root("src/b3_conversion/jsons/new2", true) {
        Ok(mut a) => {
            let ans = generate_interface(&mut a);
            std::fs::write(
                "src/b3_conversion/new2_accessor.rs",
                &ans.to_string(),
            ).unwrap();
        }
        Err(_e) => {
            assert!(false);
        }
    }
}

