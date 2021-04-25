
pub trait IdentityEqual{
    fn identity_eq(&self, other : &Self) -> bool;
}

// impl<K,  V : IdentityEqual, S> IdentityEqual for HashMap<K,V,S>
//     where K: Eq + Hash,
//           S: BuildHasher{
//
//     fn identity_eq(&self, other: &Self) -> bool {
//         if self.len() != other.len() {
//             return false;
//         }
//
//         self.iter().all(move |(key, value)| other.get(key).map_or(false, |v| value.identity_eq(v)))
//     }
// }

impl IdentityEqual for bool{
    fn identity_eq(&self, other: &Self) -> bool {
        self == other
    }
}

impl IdentityEqual for i64{
    fn identity_eq(&self, other: &Self) -> bool {
        self == other
    }
}

impl IdentityEqual for f64{
    fn identity_eq(&self, other: &Self) -> bool {
        self == other
    }
}