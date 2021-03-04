#!/bin/bash

set -e

export RUSTFLAGS="-Clto -Cembed-bitcode=yes --emit=llvm-bc $RUSTFLAGS"
export RUSTFLAGS="--cfg=verify $RUSTFLAGS"
export RUSTFLAGS="-Warithmetic-overflow -Coverflow-checks=yes $RUSTFLAGS"
export RUSTFLAGS="--cfg=verify $RUSTFLAGS"
export RUSTFLAGS="-Warithmetic-overflow -Coverflow-checks=yes $RUSTFLAGS"
export RUSTFLAGS="-Zpanic_abort_tests -Cpanic=abort $RUSTFLAGS"

# optional for this simple example
export RUSTFLAGS="-Copt-level=1 $RUSTFLAGS"
export RUSTFLAGS="-Cno-vectorize-loops -Cno-vectorize-slp $RUSTFLAGS"
export RUSTFLAGS="-Ctarget-feature=-mmx,-sse,-sse2,-sse3,-ssse3,-sse4.1,-sse4.2,-3dnow,-3dnowa,-avx,-avx2 $RUSTFLAGS"

# cargo clean
cargo build --features=verifier-klee

# verify using KLEE
rm -rf kleeout
klee ${KLEE_FLAGS} --libc=klee --silent-klee-assume --output-dir=kleeout --warnings-only-to-file target/debug/deps/try_klee*.bc

for i in kleeout/test*.ktest
do
  echo "Showing test file $i"

  # view input value for first path
  # ktest-tool $i | grep ' int '

  # replay input values
  KTEST_FILE=$i cargo run --features=verifier-klee --quiet
done