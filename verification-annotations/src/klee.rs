// Copyright 2020 The Propverify authors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/////////////////////////////////////////////////////////////////
// FFI wrapper for KLEE symbolic execution tool
/////////////////////////////////////////////////////////////////

pub use crate::traits::*;

use std::default::Default;
use std::ffi::CString;
use std::mem;
use std::os::raw;

#[link(name = "kleeRuntest")]
extern "C" {
    fn klee_make_symbolic(data: *mut raw::c_void, length: usize, name: *const raw::c_char);
    fn klee_assume(cond: usize);
    fn klee_abort() -> !;
    fn klee_silent_exit(_ignored: u32) -> !;
    fn klee_is_replay() -> u32;
    fn klee_is_symbolic(x: usize) -> bool;
    fn klee_choose(x: usize) -> usize;
    fn klee_get_value_i32(x: i32) -> i32;
    fn klee_get_value_i64(x: i64) -> i64;
    fn klee_open_merge();
    fn klee_close_merge();
}

/// Create instance for any type consisting of contiguous memory
/// where all bit-patterns are legal values of the type.
macro_rules! make_verifier_nondet {
    ($typ:ident) => {
        impl VerifierNonDet for $typ {
            fn verifier_nondet(self) -> Self {
                let mut r = self;
                unsafe {
                    let data = std::mem::transmute(&mut r);
                    let length = std::mem::size_of::<$typ>();
                    let null = 0 as *const i8;
                    klee_make_symbolic(data, length, null)
                }
                return r;
            }
        }
    };
}

make_verifier_nondet!(u8);
make_verifier_nondet!(u16);
make_verifier_nondet!(u32);
make_verifier_nondet!(u64);
make_verifier_nondet!(u128);
make_verifier_nondet!(usize);

make_verifier_nondet!(i8);
make_verifier_nondet!(i16);
make_verifier_nondet!(i32);
make_verifier_nondet!(i64);
make_verifier_nondet!(i128);
make_verifier_nondet!(isize);

make_verifier_nondet!(f32);
make_verifier_nondet!(f64);

/// Allocate a symbolic vector of bytes
pub fn verifier_nondet_bytes(n: usize) -> Vec<u8> {
    // create an empty vector with capacity 'n'
    let v: Vec<u8> = Vec::with_capacity(n);

    // take vector apart so that we can access unused capacity
    let mut v = mem::ManuallyDrop::new(v);
    let p = v.as_mut_ptr();

    unsafe {
        // mark contents of vector as symbolic
        let data = p as *mut raw::c_void;
        let null = 0 as *const i8;
        klee_make_symbolic(data, n, null);

        // put vector back together again,
        // with same capacity but increased length
        Vec::from_raw_parts(p, n, n)
    }
}

/// Allocate a symbolic CString
pub fn verifier_nondet_cstring(size_excluding_null: usize) -> CString {
    let mut r = verifier_nondet_bytes(size_excluding_null + 1);
    for i in 0..size_excluding_null {
        assume(r[i] != 0u8);
    }
    r[size_excluding_null] = 0u8;
    unsafe { CString::from_vec_with_nul_unchecked(r) }
}

/// Allocate a symbolic ASCII String
/// (ASCII strings avoid the complexity of UTF-8)
pub fn verifier_nondet_ascii_string(n: usize) -> String {
    let r = verifier_nondet_bytes(n);
    for i in 0..n {
        assume(r[i] != 0u8);
        assume(r[i].is_ascii());
    }
    match String::from_utf8(r) {
        Ok(r) => r,
        Err(_) => reject(),
    }
}

impl VerifierNonDet for bool {
    fn verifier_nondet(self) -> Self {
        let c = VerifierNonDet::verifier_nondet(0u8);
        assume(c == 0 || c == 1);
        c == 1
    }
}

impl <T: VerifierNonDet + Default> AbstractValue for T {
    fn abstract_value() -> Self {
        Self::verifier_nondet(Self::default())
    }
}

impl <T: VerifierNonDet + Default> Symbolic for T {
    fn symbolic(_desc: &'static str) -> Self {
        Self::verifier_nondet(Self::default())
    }
}

/// Assume that condition `cond` is true
///
/// Any paths found must satisfy this assumption.
pub fn assume(cond: bool) {
    unsafe { klee_assume(if cond { 1 } else { 0 }) }
}

/// Reject the current execution with a verification failure.
///
/// In almost all circumstances, `report_error` should
/// be used instead because it generates an error message.
pub fn abort() -> ! {
    unsafe { klee_abort() }
}

/// Reject the current execution path with a verification success.
/// This is equivalent to `assume(false)`
/// and the opposite of `report_error(...)`.
///
/// Typical usage is in generating symbolic values when the value
/// does not meet some criteria.
pub fn reject() -> ! {
    unsafe { klee_silent_exit(0) }
}

/// Detect whether the program is being run symbolically in KLEE
/// or being replayed using the kleeRuntest runtime.
///
/// This is used to decide whether to display the values of
/// variables that may be either symbolic or concrete.
pub fn is_replay() -> bool {
    unsafe { klee_is_replay() != 0 }
}

pub fn is_symbolic_u32(x: u32) -> bool {
    unsafe { klee_is_symbolic(x as usize) }
}

pub fn get_value_i32(x: i32) -> i32 {
    unsafe { klee_get_value_i32(x) }
}

pub fn get_value_i64(x: i64) -> i64 {
    unsafe { klee_get_value_i64(x) }
}

pub fn get_value_u32(x: u32) -> u32 {
    get_value_i32(x as i32) as u32
}

pub fn get_value_u64(x: u64) -> u64 {
    get_value_i64(x as i64) as u64
}

/// Open a merge block
///
/// Should be paired with `close_merge`
pub fn open_merge() {
    // safe because it only affects KLEE scheduling
    unsafe {
        klee_open_merge();
    }
}

/// Close a merge block
///
/// Should be paired with `open_merge`
pub fn close_merge() {
    // safe because it only affects KLEE scheduling
    unsafe {
        klee_close_merge();
    }
}

/// Coherent blocks don't fork execution during verification.
///
/// This will only take effect if executed with the
/// KLEE command line flags `--use-merge` and, optionally,
/// `--use-incomplete-merge`.
///
/// This might reduce the number of instructions that KLEE explores
/// because there are less forks.
/// This might also make evaluation of the symbolic constraints
/// more expensive because of state merging.
///
/// Caveats:
/// - Branches out of the middle of `$body` such as return, etc. will not be merged.
///   If this is a problem, you should use open_merge/close_merge explicitly.
///
/// - If the body performs memory allocation, merging cannot happen.
#[macro_export]
macro_rules! coherent {
    ( $body:block ) => {
        $crate::open_merge();
        $body;
        $crate::close_merge();
    };
}

/// Reject the current execution with a verification failure
/// and an error message.
pub fn report_error(message: &str) -> ! {
    // Mimic the format of klee_report_error
    // (We don't use klee_report_error because it is not
    // supported by the kleeRuntest library.)
    eprintln!("KLEE: ERROR:{}", message);
    abort();
}

/// Split exploration into two cases: one satisfying `x`, one not.
///
/// This is one way of forcing KLEE to make a case split and
/// concretize a choice.
pub fn case_split(x: bool) {
    if x {
        // dummy call to prevent compiler from deleting this call
        // this will generate an error message that you can ignore
        crate::verifier_use_u32(0)
    }
}

/// Split exploration into one case for each value in xs.
///
/// This is one way of forcing KLEE to make a case split and
/// concretize a choice.
pub fn any_of<T>(xs: &[T]) -> &T {
    let n = xs.len();
    if n == 0 {
        report_error("verifier::choose() empty choice")
    }
    let c = unsafe { klee_choose(n) };
    for (i, x) in xs.iter().enumerate() {
        if i == c {
            return x;
        }
    }
    report_error("verifier::choose(): internal error: failed to choose value")
}

/// Split exploration into `samples` cases by choosing a distinct concrete solution
/// for `x` in each case.
///
/// In most cases, this results in an incomplete exploration because there may
/// be more possible solutions than we explore.
pub fn sample_u32(samples: usize, x: u32) -> u32 {
    for _i in 0..samples-1 {
        let s = get_value_u32(x);
        if s == x {
            return s;
        }
    }
    get_value_u32(x)
}

/// Exhaustively enumerate all possible concrete values for `x`.
///
/// If there are a finite number of possible values for `x`,
/// this terminates because get_value_u32 terminates if there are
/// no further solutions.
pub fn concretize_u32(x: u32) -> u32 {
    loop {
        let s = get_value_u32(x);
        if s == x {
            return s;
        }
    }
}

pub fn concretize_usize(x: usize) -> usize {
    loop {
        let s = get_value_u64(x as u64) as usize;
        if s == x {
            return s;
        }
    }
}

fn bit(i: u32, x: u32) -> u32 {
    (x >> i) & 1
}

#[allow(dead_code)]
fn hash0(x: u32) -> u32 {
    let x0 = bit(3, x) ^ bit(5, x) ^ bit(6, x);
    let x1 = bit(1, x) ^ bit(2, x) ^ bit(4, x);
    let x2 = bit(0, x) ^ bit(4, x) ^ bit(6, x);
    let x3 = bit(2, x) ^ bit(7, x) ^ bit(8, x);
    let x4 = bit(1, x) ^ bit(3, x) ^ bit(9, x);
    x0 | (x1 << 1) | (x2 << 2) | (x3 << 3) | (x4 << 4)
}

// todo: this should be replaced with a universal hash function
// (https://en.wikipedia.org/wiki/Universal_hashing)
#[allow(dead_code)]
fn hash1(x: u32) -> u32 {
    u32::wrapping_add(u32::wrapping_mul(x, 1664525), 1013904223)
}

// The standard sampling technique (used in `sample_u32`) produces
// tightly clustered values.
// This is unfortunate because sampling is used to switch from
// verification to testing and, for testing, we want to maximize
// diversity.
//
// This variant uses ideas from uniform random sampling with SAT/SMT to
// generate more widely distributed solutions.
// This relies on using random hash functions from a family of
// uniform hash functions.
// At the moment, the hash functions used are neither random nor uniform.
pub fn random_sample_u32(log_samples: usize, x: u32) -> u32 {
    let samples = 1 << log_samples;
    let h = hash0(x) & (samples - 1);
    for i in 0..samples {
        if i == h {
            let s = get_value_u32(x);
            assume(s == x);
            return s;
        }
    }
    // todo: could also report an error here since one must have matched
    reject()
}

/// Declare that failure is the expected behaviour
pub fn expect_raw(msg: &str) {
    eprintln!("VERIFIER_EXPECT: {}", msg)
}

/// Declare that failure is the expected behaviour
pub fn expect(msg: Option<&str>) {
    match msg {
        None => eprintln!("VERIFIER_EXPECT: should_panic"),
        Some(msg) => eprintln!("VERIFIER_EXPECT: should_panic(expected = \"{}\")", msg)
    }
}


#[macro_export]
macro_rules! assert {
    ($cond:expr,) => { $crate::assert!($cond) };
    ($cond:expr) => { $crate::assert!($cond, "assertion failed: {}", stringify!($cond)) };
    ($cond:expr, $($arg:tt)+) => {{
        if ! $cond {
            let message = format!($($arg)+);
            eprintln!("VERIFIER: panicked at '{}', {}:{}:{}",
                      message,
                      std::file!(), std::line!(), std::column!());
            $crate::abort();
        }
    }}
}

#[macro_export]
macro_rules! assert_eq {
    ($left:expr, $right:expr) => {{
        let left = $left;
        let right = $right;
        $crate::assert!(
            left == right,
            "assertion failed: `(left == right)` \
             \n  left: `{:?}`,\n right: `{:?}`",
            left,
            right)
    }};
    ($left:expr, $right:expr, $fmt:tt $($arg:tt)*) => {{
        let left = $left;
        let right = $right;
        $crate::assert!(
            left == right,
            concat!(
                "assertion failed: `(left == right)` \
                 \n  left: `{:?}`, \n right: `{:?}`: ", $fmt),
            left, right $($arg)*);
    }};
}

#[macro_export]
macro_rules! assert_ne {
    ($left:expr, $right:expr) => {{
        let left = $left;
        let right = $right;
        $crate::assert!(
            left != right,
            "assertion failed: `(left != right)` \
             \n  left: `{:?}`,\n right: `{:?}`",
            left,
            right)
    }};
    ($left:expr, $right:expr, $fmt:tt $($arg:tt)*) => {{
        let left = $left;
        let right = $right;
        $crate::assert!(
            left != right,
            concat!(
                "assertion failed: `(left != right)` \
                 \n  left: `{:?}`, \n right: `{:?}`: ", $fmt),
            left, right $($arg)*);
    }};
}

/////////////////////////////////////////////////////////////////
// End
/////////////////////////////////////////////////////////////////
