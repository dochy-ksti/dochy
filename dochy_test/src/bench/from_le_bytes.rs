// pub(crate) fn from_le_bytes_bench() {
//     let mut random = rand::thread_rng();
//
//     let len = 800_000_000;
//     let mut vec: Vec<u8> = Vec::with_capacity(len);
//
//     for _ in 0..len {
//         vec.push(random.gen());
//     }
//
//     let len64 = len / 8;
//
//     let mut vec64: Vec<u64> = Vec::with_capacity(len64);
//
//     for _ in 0..len64 {
//         vec64.push(random.gen());
//     }
//
//     for _ in 0..3 {
//         let mut w = Vec::with_capacity(len);
//         run64(&vec64, &mut w);
//     }
//
//     for _ in 0..3 {
//         let mut w = Vec::with_capacity(len);
//         run(&vec, &mut w);
//     }
// }
//
//     fn run64<W : Write>(vec : &Vec<u64>, writer : &mut W) {
//         let start_time = Instant::now();
//         for v in vec{
//             let value = u64::to_le_bytes(*v);
//             writer.write_all(&value).unwrap();
//         }
//         let time = (Instant::now() - start_time).as_millis() as u64;
//
//         println!("run64 {} millis", time);
//
//     }
//
//     fn run<W : Write>(vec : &Vec<u8>, writer : &mut W) {
//         let start_time = Instant::now();
//
//         writer.write_all(&vec).unwrap();
//
//         let time = (Instant::now() - start_time).as_millis() as u64;
//
//         println!("run {} millis", time);
//
//     } from
//
// //run64 406 millis
// //run64 369 millis
// //run64 342 millis
// //run 197 millis
// //run 193 millis
// //run 181 millis
