use std::time::{SystemTime, UNIX_EPOCH};
use rand::Rng;
use crate::imp::structs::util::identity_equal_trait::IdentityEqual;

/// Every time an big array is set to an object, generates this and set it with the array.
/// When time and random value is the same,
/// we can presume it's not modified without comparing actual values.
/// It's not 100%, but with working clock and decent random generator,
/// I believe false negatives are negligible.
///
/// time's unit is 100 nano seconds.
/// Unless you update, save and update within 100 nano second,
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
        // 1600京回ぐらいやれば60%ぐらいの確率で発生する
        let random = rand::thread_rng().gen();
        //100ナノ秒を単位にする
        //u64で3万年ぐらいの期間を表せる
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| (d.as_nanos() / 100) as u64)
            .unwrap_or(0);
        RustIdentity{ time, random }
    }
    pub fn create(time : u64, random : u64) -> RustIdentity{
        RustIdentity{ time, random }
    }
    pub fn time(&self) -> u64{ self.time }
    pub fn random(&self) -> u64{ self.random }
}

impl IdentityEqual for RustIdentity{
    fn identity_eq(&self, other: &Self) -> bool {
        self == other
    }
}