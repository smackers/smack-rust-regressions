#[macro_use]
mod smack;
use smack::*;

use std::slice;

extern {
  fn get_numbers(length: usize) -> *mut u32;
}

fn main() {
  let len = 10usize.nondet();
  let mut x = unsafe { slice::from_raw_parts_mut(get_numbers(len), len) };
  let mut sum = 0;
  for i in 0..len {
    sum += x[i];
    x[i] = sum;
  }
}
