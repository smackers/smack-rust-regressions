#[macro_use]
mod smack;
use smack::*;
// @expect error
fn main() {
  let a = 2;
  let b = 3;
  assert!(a+b != 5);
}
