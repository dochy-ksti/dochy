#[cfg(test)]
mod tests {
    use crate::core::json_dir_to_rust;
    use crate::intf::generate_interface;
    use crate::intf::test::write_file::test::write_file;
    //use crate::intf::rust_to_json_new_default;
    //use crate::intf::imp::json_to_rust::json_root_to_rust;
    //use crate::intf::imp::rust_to_json::root_to_json::root_to_json_new_default;

    #[test]
    fn it_works() {
        match json_dir_to_rust("src/json_dir/test/siyou", true) {
            Ok(mut a) => {
                //println!("{:?}", a);
                let ans = generate_interface(&mut a);
                let source = ans.to_string();
                write_file("src/test_generated/siyou.rs", &source);
            },
            Err(e) => { println!("val 1 {}", e.to_string()) }
        }
    }
}