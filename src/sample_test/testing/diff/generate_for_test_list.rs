

#[cfg(test)]
mod tests {
    use crate::sample_test::testing::diff::util::generate_intf_src::generate_intf_src;
    use crate::sample_test::error::DpResult;

    #[test]
    fn generate_for_test_ref(){
        match generate_for_test_ref2(){
            Ok(()) =>{},
            Err(s) => println!("{}", s),
        }


    }

    fn generate_for_test_ref2() -> DpResult<()>{
        let json_dir_path = "src/testing/diff/diff_list/";
        let _root_obj = generate_intf_src(json_dir_path, "src/testing/diff/generated_test_list.rs")?;
        Ok(())
    }


}