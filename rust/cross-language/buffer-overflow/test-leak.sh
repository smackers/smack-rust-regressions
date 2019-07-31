#! /bin/bash

# Run this to make sure the smack mod is present
smack consumer-leak.rs --no-verify
rustc -A unused-imports -C opt-level=0 -C no-prepopulate-passes -g --emit=llvm-ir --cfg 'verifier="smack"' consumer-leak.rs
clang -S -emit-llvm producer-correct.c
llvm-link consumer-leak.ll producer-correct.ll -o buffer-overflow-leak.bc
smack buffer-overflow-leak.bc --unroll=4 --memory-safety -bpl out.bpl

