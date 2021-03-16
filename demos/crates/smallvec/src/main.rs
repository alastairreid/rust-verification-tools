/// Test smallvec crate
///
/// cargo verify --tests -v --script=log --replay --backend-flags=--search=dfs
/// for i in kleeout/test_smallvec/test*.ktest; do KTEST_FILE=$i cargo test --features verifier-klee test_smallvec -- --nocapture; done
///
/// These tests only check that the library does not throw exceptions / panic.
/// They do not check that the functions behave as intended.

use verification_annotations as verifier;
use smallvec::*;

const N: usize = 27;

#[test]
fn test_smallvec() {
    let mut backing = verifier::verifier_nondet_bytes(N);
    let mut v: SmallVec<[u8; N]> = SmallVec::from_vec(backing);
    v.extend(0..(N+10) as u8); // trigger an overflow
}
