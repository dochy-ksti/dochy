use std::fmt::{Display, Formatter};
use crate::imp::structs::util::identity_equal_trait::IdentityEqual;

#[derive(Debug, Clone)]
pub struct RustString{
    str : Box<String>,
}

impl RustString{
    pub fn new(s : String) -> RustString{ RustString{ str : Box::new(s) } }
    pub fn str(&self) -> &str{ self.str.as_ref().as_str() }
    pub fn str_mut(&mut self) -> &mut str{ self.str.as_mut().as_mut_str() }
    pub fn string(&self) -> &String{ self.str.as_ref() }
    pub fn string_mut(&mut self) -> &mut String{ self.str.as_mut() }
}

impl Display for RustString{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.str().fmt(f)
    }
}

impl IdentityEqual for RustString{
    fn identity_eq(&self, other: &Self) -> bool {
        self.str == other.str
    }
}



// #[derive(Debug, Clone)]
// pub struct RustBigString{
//     str : Box<(String, RustIdentity)>,
// }
//
// impl RustBigString{
//     pub fn new(s : String) -> RustBigString{ RustBigString{ str : Box::new((s, RustIdentity::new())) } }
//     pub fn str(&self) -> &str{ self.str.as_ref().0.as_str() }
//
// }
//
// impl Display for RustBigString{
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         self.str().fmt(f)
//     }
// }
//
// impl IdentityEqual for RustBigString{
//     fn identity_eq(&self, other: &Self) -> bool {
//         self.str.as_ref().1 == other.str.as_ref().1
//     }
// }



