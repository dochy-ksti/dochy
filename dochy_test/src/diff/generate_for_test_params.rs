

#[cfg(test)]
mod tests {
    use dochy::error::DpResult;
    use crate::diff::util::generate_intf_src::generate_intf_src;

    #[test]
    fn generate_for_test_params2() -> DpResult<()>{
        let json_dir_path = "src/diff/diff_param/";
        let _root_obj = generate_intf_src(json_dir_path, "src/diff/generated_test_params.rs")?;
        Ok(())
    }


}