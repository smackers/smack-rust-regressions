#[macro_use]
mod smack;
use smack::*;

extern {fn incr(i: *mut u64) -> ();}

fn main() {
    let mut i = 0u64.nondet();
    let old_i = i;
    unsafe{incr(&mut i)};
    assert!(i==old_i+1);
}
