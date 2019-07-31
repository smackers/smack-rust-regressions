#! /bin/bash

# Run this to make sure the smack mod is present
smack incr_test_fail.rs --no-verify
rustc -A unused-imports -C opt-level=0 -C no-prepopulate-passes -g --emit=llvm-ir --cfg 'verifier="smack"' incr_test_fail.rs
clang -S -emit-llvm incr.c
llvm-link incr_test_fail.ll incr.ll -o incr_fail.bc
smack incr_fail.bc --no-memory-splitting
