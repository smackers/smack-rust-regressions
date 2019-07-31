#[macro_use]
mod smack;
use smack::*;

// Declare the C function
extern { fn fib_c(n: u64) -> u64; }

fn fib(x: usize, cache: &mut Vec<u64>) {
  for i in 2..x+1 as usize {
    cache[i] = cache[i-1] + cache[i-2];
  }
}

fn main() {
  let n = 5u64.nondet();
  assume!(n > 2);
  let mut cache = vec![0; n+1];
  cache[0] = 0;
  cache[1] = 1;
  fib(n as usize, &mut cache);
  let c_result = unsafe{ fib_c(n as u64) };
  assert!(cache[n as usize] != c_result);
}
