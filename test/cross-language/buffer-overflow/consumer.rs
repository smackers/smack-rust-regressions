#[macro_use]
mod smack;
use smack::*;

use std::slice;

extern {
  fn get_numbers(length: usize) -> *mut u32;
  fn delete_numbers(ptr: *mut u32);
}

fn main() {
  let len = 10usize.nondet();
  let buffer = unsafe { get_numbers(len) };
  {
    let mut x = unsafe { slice::from_raw_parts_mut(buffer, len) };
    let mut sum = 0;
    for i in 0..len {
      sum += x[i];
      x[i] = sum;
    }
  }
  unsafe { delete_numbers(buffer) };
}
