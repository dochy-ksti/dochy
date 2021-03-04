//
// pub(crate ) struct GeneralIter<'a, T,U>{
//     len : usize,
//     counter : usize,
//     intf : &'a T,
//     getter : fn(&T,usize) -> U,
// }
// impl<'a, T : 'a,U> GeneralIter<'a, T,U>{
//     pub fn new(len : usize, intf : &'a T, getter : fn(&T,usize) -> U) -> GeneralIter<'a, T,U>{
//         GeneralIter{ len, counter : 0, intf, getter }
//     }
// }
//
// impl<'a, T : 'a,U> Iterator for GeneralIter<'a, T, U>{
//     type Item = U;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         if self.counter < self.len{
//             let counter = self.counter;
//             self.counter += 1;
//             Some((self.getter)(self.intf, counter))
//         } else{
//             None
//         }
//     }
// }