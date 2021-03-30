

#[cfg(test)]
mod tests {
    use crate::testing::diff::util::generate_intf_src::generate_intf_src;
    use crate::error::DpResult;

    #[test]
    fn generate_for_test_params(){
        match generate_for_test_params2(){
            Ok(()) =>{},
            Err(s) => println!("{}", s),
        }


    }

    fn generate_for_test_params2() -> DpResult<()>{
        let json_dir_path = "src/testing/diff/diff_param/";
        let _root_obj = generate_intf_src(json_dir_path, "src/testing/diff/generated_test_params.rs")?;
        Ok(())
    }


}