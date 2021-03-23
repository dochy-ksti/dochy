// #[cfg(test)]
// mod tests {
//     use rand::prelude::ThreadRng;
//     use rand::Rng;
//
//     use test::Bencher;
//     use std::collections::{HashMap, BTreeMap};
//     use linked_hash_map::LinkedHashMap;
//
//     const NAME_LEN : usize = 8;
//     const NUM_NAMES : usize = 1_000_0;
//
//     struct VeryBig{
//         map1 : HashMap<String, u64>, map2 : HashMap<String, u64>, map3 : HashMap<String, u64>,
//         //map4 : HashMap<String, u64>, map5 : HashMap<String, u64>, map6 : HashMap<String, u64>,
//         //map7 : HashMap<String, u64>, map8 : HashMap<String, u64>, map9 : HashMap<String, u64>,
//         //map10 : HashMap<String, u64>, map11 : HashMap<String, u64>, map12 : HashMap<String, u64>,
//     }
//
//     impl VeryBig{
//         pub fn new() -> VeryBig{
//             VeryBig{
//                 map1 : HashMap::new(), map2 : HashMap::new(), map3 :HashMap::new(),
//                 //map4 : HashMap::new(), map5 : HashMap::new(), map6 :HashMap::new(),
//                 //map7 : HashMap::new(), map8 : HashMap::new(), map9 :HashMap::new(),
//                 //map10 : HashMap::new(), map11 : HashMap::new(), map12 :HashMap::new()
//             }
//         }
//     }
//
//
//     fn init1() -> (ThreadRng, Vec<String>) {
//         let rng = rand::thread_rng();
//
//         let mut names : Vec<String> = vec![];
//
//         for _ in 0..NUM_NAMES{
//             names.push(get_random_name(NAME_LEN, rng));
//         }
//
//         (rng, names)
//     }
//
//     fn get_random_name(len : usize, mut rng : ThreadRng) -> String{
//         let mut s = String::with_capacity(len);
//         for _i in 0..len {
//             s.push(rng.gen_range('a' as u8, 'z' as u8) as char );
//         }
//         return s;
//     }
//
//
//
//     #[bench]
//     fn bench_init_vec(b: &mut Bencher) {
//         let (_rng, names) = init1();
//
//         b.iter(|| {
//             let mut vec : Vec<(String, VeryBig)> = Vec::new();
//             for name in &names{
//                 vec.push((name.to_string(), VeryBig::new()));
//             }
//
//             vec.into_iter().collect::<HashMap<String, VeryBig>>()
//         });
//     }
//
//     #[bench]
//     fn bench_init_boxed_vec(b: &mut Bencher) {
//         let (_rng, names) = init1();
//
//         b.iter(|| {
//             let mut vec : Vec<Box<(String, VeryBig)>> = Vec::new();
//             for name in &names{
//                 vec.push(Box::new((name.to_string(), VeryBig::new())));
//             }
//             vec.into_iter().map(|b|*b).collect::<HashMap<String, VeryBig>>()
//         });
//     }
//
//     #[bench]
//     fn bench_init_a_set(b: &mut Bencher) {
//         let (_rng, names) = init1();
//
//         b.iter(|| {
//             let mut set : BTreeMap<String, VeryBig> = BTreeMap::new();
//             for name in &names{
//                 set.insert(name.to_string(), VeryBig::new());
//             }
//             set
//         });
//     }
//
//     #[bench]
//     fn bench_init_hash_map(b: &mut Bencher) {
//         let (_rng, names) = init1();
//
//         b.iter(|| {
//             let mut set : HashMap<String, VeryBig> = HashMap::new();
//             for name in &names{
//                 set.insert(name.to_string(), VeryBig::new());
//             }
//             set
//         });
//     }
//
//     #[bench]
//     fn bench_init_linked_hash_map(b: &mut Bencher) {
//         let (_rng, names) = init1();
//
//         b.iter(|| {
//             let mut set : LinkedHashMap<String, VeryBig> = LinkedHashMap::new();
//             for name in &names{
//                 set.insert(name.to_string(), VeryBig::new());
//             }
//             set
//         });
//     }
// }