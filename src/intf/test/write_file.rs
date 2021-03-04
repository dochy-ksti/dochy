
#[cfg(test)]
pub(crate ) mod test {
    use std::fs::File;
    use std::io::prelude::*;
    use std::path::Path;

    pub(crate ) fn write_file(file_path: &str, contents: &str) {
        // Create a path to the desired file
        let path = Path::new(file_path);
        let display = path.display();

        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't open {}: {}", display, why),
            Ok(file) => file,
        };
        match file.write_all(contents.as_bytes()) {
            Ok(()) => {},
            Err(e) => { panic!("write failed {}", e); }
        }
    }
}