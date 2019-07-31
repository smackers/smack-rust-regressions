#[macro_use]
mod smack;
use smack::*;
// @flag --no-memory-splitting --unroll=3
// @expect verified
fn main() {
   let mut v1:Vec<u64> = vec![0,1,2];
   let mut v2:Vec<u64> = vec![3,4];
   v1.append(&mut v2);
   assert!(v1[4] == 4);
}