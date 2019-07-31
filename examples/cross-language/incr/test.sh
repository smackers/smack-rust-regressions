#! /bin/bash

# Run this to make sure the smack mod is present
smack incr_test.rs --no-verify
rustc -A unused-imports -C opt-level=0 -C no-prepopulate-passes -g --emit=llvm-ir --cfg 'verifier="smack"' incr_test.rs
clang -S -emit-llvm incr.c
llvm-link incr_test.ll incr.ll -o incr.bc
smack incr.bc --no-memory-splitting
