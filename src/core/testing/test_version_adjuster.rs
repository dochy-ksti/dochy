

#[cfg(test)]
mod testion {
    use crate::core::{json_dir_to_rust, rust_to_json_new_default};
    use crate::core::imp::version_adjuster::version_adjuster::adjust_versions;
    use crate::core::error::{CoreResult};
    use crate::core::imp::json_to_rust::json_root_to_rust;

    //#[test]
    pub(crate ) fn test_version_adjuster() -> CoreResult<()> {
        let new = match json_dir_to_rust("src/json_dir/version_adjuster/new", true) {
            Ok(j) => j,
            Err(e) => {
                println!("new {}", e);
                return Ok(());
            }
        };

        let old = match json_dir_to_rust("src/json_dir/version_adjuster/old", true) {
            Ok(j) =>{
                match rust_to_json_new_default(&j){
                    Ok(_b) =>{
                        //println!("old pretty {}", b.to_string_pretty());
                    },
                    Err(e) =>{ println!("old nopretty {}", e); }
                }
                j
            },
            Err(e) => {
                println!("old {}", e);
                return Ok(());
            }
        };

        match adjust_versions(new, old, true) {
            Ok(a) => {
                match rust_to_json_new_default(&a) {
                    Ok(b) => {
                        println!("pretty {}", b.to_string_pretty());
                        match json_root_to_rust(&b.to_string_pretty()){
                            Ok(a) =>{
                                match rust_to_json_new_default(&a){
                                    Ok(json2) =>{
                                        assert_eq!(b.to_string_pretty(), json2.to_string_pretty());
                                        println!("OK");
                                    },
                                    Err(e) =>{
                                        println!("ERR 2 {}", e);
                                    }
                                }

                            }
                            Err(e) =>{ println!("ERR {}", e)}
                        }
                    },
                    Err(e) => {
                        println!("rust to json {}", e);
                    }
                }
            }
            Err(e) => { println!("adjust versions {}", e); }
        }
        return Ok(());
    }
}