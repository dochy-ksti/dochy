use std::time::SystemTime;
use rand::Rng;

/// Every time an array is set, generates this and set with the array.
/// When SystemTime and random value is the same,
/// we can presume it's not modified without comparing actual values.
/// It's not 100%, but with working clock and decent random generator,
/// I believe false negatives are negligible.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RustIdentity{
    time : SystemTime,
    random : u128,
}

impl RustIdentity{
    pub fn new() -> RustIdentity{
        let random : u128 = rand::thread_rng().gen();
        let time = SystemTime::now();
        RustIdentity{ time, random }
    }
}