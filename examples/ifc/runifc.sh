#! /bin/bash

echo "-----------------------------------------------------------------------"
echo "Running ifc-full"
echo
echo "-----------------------------------------------------------------------"
echo "Running ifc"
echo "Result:"
smack ifc_correct/ifc.rs --no-memory-splitting --unroll=4 --time-limit=21600 |& tail -1
echo "Expected: SMACK found no errors with unroll bound 1."

echo "-----------------------------------------------------------------------"
echo "Running ifc_bug"
echo "Result:"
smack ifc_bug/ifc_bug.rs --no-memory-splitting --unroll=4 --time-limit=21600 |& tail -1
echo "Expected: SMACK found an error."
