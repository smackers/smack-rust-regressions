#[macro_use]
#[macro_export]
pub mod smack;
use smack::*;
// @flag --no-memory-splitting --rust-options '--cfg feature="verify"'
// @expect verified
pub mod label {
    use std::cmp::max;
    pub type Label = u64;
    pub fn combine_labels(l1: Label, l2: Label) -> Label {
        max(l1,l2)
    }
}

pub use self::verify::*;
#[macro_use]
mod verify;
pub mod sec_vec;
use sec_vec::sec_vec::*;
//use sec_vec::SecVec;

/* For verification a main function is needed. This is from the sec_vec/mod.rs
   test.
*/
fn main() {

  let nd1 = 5u64.nondet();
  let nd2 = 6u64.nondet();

  let nd3 = 5u64.nondet();
  let nd4 = 6u64.nondet();

  // Maintain permission invariant
  assume!(nd1 < nd2);  // Create permissions; nd1 is low authority
                       // nd2 is high authority
  assume!(nd3 < nd2);  // nd3 is used to read with low authority
  assume!(nd4 == nd2); // nd4 is the high authority access

  let mut s = SecVec::new();
  let lsecret = svec![1,2,3 => nd1];
  let hsecret = svec![4,5,6 => nd2];

//  println!("Adding secrets");
  s.push(lsecret, nd1);
  s.update(0, hsecret, nd2);

//  println!("s: {:?}", s);

//  println!("Reading secrets with low authority");
  match s.get(0, nd3) {
    None    => assert!(true), // println!("Access forbidden"), // Correcly reject access
    Some(v) => assert!(false) // check_label!(v,nd3), // Incorrectly allow access
  }

//  println!("Reading secrets with high authority");
  match s.get(0, nd4) {
    None      => assert!(false), // println!("Access forbidden"), // Rejected proper access
    Some(sec) => {assert!(true); // Correctly grant access
      check_label!(sec,nd4);
      //println!("Secret value: {:?}", sec);
    }
  }
}
