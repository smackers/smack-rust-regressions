#! /bin/bash

# Run this to make sure the smack mod is present
smack consumer.rs --no-verify
rustc -A unused-imports -C opt-level=0 -C no-prepopulate-passes -g --emit=llvm-ir --cfg 'verifier="smack"' consumer.rs
clang -S -emit-llvm producer-correct.c
llvm-link consumer.ll producer-correct.ll -o buffer-overflow.bc
smack buffer-overflow.bc --unroll=4 --memory-safety -bpl out.bpl

