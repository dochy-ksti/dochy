use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub struct RustString{
    str : Box<String>,
}

impl RustString{
    pub fn new(s : String) -> RustString{ RustString{ str : Box::new(s) } }
    pub fn str(&self) -> &str{ self.str.as_ref().as_str() }

}

impl Display for RustString{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.str().fmt(f)
    }
}



