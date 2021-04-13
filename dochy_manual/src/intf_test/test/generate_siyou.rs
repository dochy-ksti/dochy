#[cfg(test)]
mod tests {
    use dochy::core::json_dir_to_root;
    use dochy::intf::generate_interface;
    use dochy::error::DpResult;

    //use crate::rust_to_json_new_default;
    //use crate::imp::json_to_rust::json_root_to_rust;
    //use crate::imp::rust_to_json::root_to_json::root_to_json_new_default;

    #[test]
    fn it_works() -> DpResult<()>{

        // match json_dir_to_rust("src/test/siyou", true) {
        match json_dir_to_root("../dochy_core/src/json_dir/json_siyou", true) {
            Ok(mut a) => {
                //println!("{:?}", a);
                let ans = generate_interface(&mut a);
                let source = ans.to_string();
                std::fs::write("src/intf_test/test_generated/siyou.rs", &source)?;
            },
            Err(e) => { println!("val 1 {}", e.to_string()) }
        }
        Ok(())
    }
}