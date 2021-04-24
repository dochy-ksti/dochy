use std::time::{SystemTime, UNIX_EPOCH};
use rand::Rng;
use crate::imp::structs::util::identity_equal_trait::IdentityEqual;

/// Every time an big array is set to an object, generates this and set it with the array.
/// When time and random value is the same,
/// we can presume it's not modified without comparing actual values.
/// It's not 100%, but with working clock and decent random generator,
/// I believe false negatives are negligible.
///
/// time is micro seconds.
/// Unless you don't update and save twice in a micro second,
/// and their random numbers are accidentally the same,
/// this shouldn't be false negative.
#[derive(Debug, PartialEq, Clone)]
pub struct RustIdentity{
    time : u64,
    random : u64,
}

impl RustIdentity{
    pub fn new() -> RustIdentity{
        // これが偶然おなじになる確率が 1 / 2^64 とすると、
        // 1000兆回やれば 1/2ぐらいの確率で発生する
        let random = rand::thread_rng().gen();
        //u64で30万年ぐらいのマイクロ秒を表せる
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_micros() as u64)
            .unwrap_or(0);
        RustIdentity{ time, random }
    }
}

impl IdentityEqual for RustIdentity{
    fn identity_eq(&self, other: &Self) -> bool {
        self == other
    }
}