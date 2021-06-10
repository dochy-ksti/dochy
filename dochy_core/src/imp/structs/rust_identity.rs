use rand::Rng;
use crate::imp::structs::util::identity_equal_trait::IdentityEqual;

/// Every time an big array is set to an object, generates this and set it with the array.
/// When ID objects are the same,
/// we can presume it's not modified without comparing actual values.
#[derive(Debug, PartialEq, Clone)]
pub struct RustIdentity{
    random1 : u64,
    random2 : u64,
}

impl RustIdentity{
    /// Randomly generates an ID
    pub fn new() -> RustIdentity{
        let mut r = rand::thread_rng();
        let random1 = r.gen();
        let random2 = r.gen();

        RustIdentity{ random1, random2 }
    }
    pub fn create(random1 : u64, random2 : u64) -> RustIdentity{
        RustIdentity{ random1, random2 }
    }
    pub fn random1(&self) -> u64{ self.random1 }
    pub fn random2(&self) -> u64{ self.random2 }
}

impl IdentityEqual for RustIdentity{
    fn identity_eq(&self, other: &Self) -> bool {
        self == other
    }
}