use std::hash::Hasher;


pub(crate) fn calc_hash(data : &[u8]) -> u128{
    let mut hasher = metrohash::MetroHash128::new();
    hasher.write(data);
    let (l,r) = hasher.finish128();
    let mut r = u128::from(r);
    r |= (l as u128) << 64;
    r
}