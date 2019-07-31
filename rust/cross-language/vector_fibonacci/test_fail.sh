#! /bin/bash

# Run this to make sure the smack mod is present
smack fibonacci_fail.rs --no-verify
rustc -A unused-imports -C opt-level=0 -C no-prepopulate-passes -g --emit=llvm-ir --cfg 'verifier="smack"' fibonacci_fail.rs
clang -S -emit-llvm fibonacci_c.c
llvm-link fibonacci_fail.ll fibonacci_c.ll -o fibonacci_fail.bc
smack fibonacci_fail.bc --unroll=5 --no-memory-splitting -bpl out.bpl

