/// Test memchr crate
///
/// cargo verify --tests -v --script=log --replay --backend-flags=--search=dfs
/// for i in kleeout/memchr/test*.ktest; do KTEST_FILE=$i cargo test --features verifier-klee memchr -- --nocapture; done
///
/// These tests only check that the library does not throw exceptions / panic.
/// They do not check that the functions behave as intended.

use verification_annotations as verifier;
use memchr::*;

const N: usize = 37;

#[test]
fn test_memchr() {
    let haystack = verifier::verifier_nondet_bytes(N);
    let haystack = &haystack[..];
    let needle   = verifier::AbstractValue::abstract_value();
    let r = memchr(needle, haystack);
    if verifier::is_replay() {
        println!("memchr({:?}, {:?}) = {:?}", needle, haystack, r);
    }
}

#[test]
fn test_memrchr() {
    let haystack = verifier::verifier_nondet_bytes(N);
    let haystack = &haystack[..];
    let needle   = verifier::AbstractValue::abstract_value();
    let r = memrchr(needle, haystack);
    if verifier::is_replay() {
        println!("memrchr({:?}, {:?}) = {:?}", needle, haystack, r);
    }
}

#[test]
fn test_memchr2() {
    let haystack = verifier::verifier_nondet_bytes(N);
    let haystack = &haystack[..];
    let needle1  = verifier::AbstractValue::abstract_value();
    let needle2  = verifier::AbstractValue::abstract_value();
    let r = memchr2(needle1, needle2, haystack);
    if verifier::is_replay() {
        println!("memchr2({:?}, {:?}, {:?}) = {:?}", needle1, needle2, haystack, r);
    }
}

#[test]
fn test_memrchr2() {
    let haystack = verifier::verifier_nondet_bytes(N);
    let haystack = &haystack[..];
    let needle1  = verifier::AbstractValue::abstract_value();
    let needle2  = verifier::AbstractValue::abstract_value();
    let r = memrchr2(needle1, needle2, haystack);
    if verifier::is_replay() {
        println!("memrchr2({:?}, {:?}, {:?}) = {:?}", needle1, needle2, haystack, r);
    }
}

#[test]
fn test_memchr3() {
    let haystack = verifier::verifier_nondet_bytes(N);
    let haystack = &haystack[..];
    let needle1  = verifier::AbstractValue::abstract_value();
    let needle2  = verifier::AbstractValue::abstract_value();
    let needle3  = verifier::AbstractValue::abstract_value();
    let r = memchr3(needle1, needle2, needle3, haystack);
    if verifier::is_replay() {
        println!("memchr3({:?}, {:?}, {:?}, {:?}) = {:?}", needle1, needle2, needle3, haystack, r);
    }
}

#[test]
fn test_memrchr3() {
    let haystack = verifier::verifier_nondet_bytes(N);
    let haystack = &haystack[..];
    let needle1  = verifier::AbstractValue::abstract_value();
    let needle2  = verifier::AbstractValue::abstract_value();
    let needle3  = verifier::AbstractValue::abstract_value();
    let r = memrchr3(needle1, needle2, needle3, haystack);
    if verifier::is_replay() {
        println!("memrchr3({:?}, {:?}, {:?}, {:?}) = {:?}", needle1, needle2, needle3, haystack, r);
    }
}
