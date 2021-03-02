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
use std::os::raw;

#[link(name = "kleeRuntest")]
extern "C" {
    fn klee_make_symbolic(data: *mut raw::c_void, length: usize, name: *const raw::c_char);
    fn klee_assume(cond: usize);
    fn klee_abort() -> !;
    fn klee_silent_exit(_ignored: u32) -> !;
    fn klee_is_replay() -> u32;
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
    ( $($body:stmt;)+ ) => {
        $crate::open_merge();
        $($body;)+
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
        // crate::verifier_use_u32(0)
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

// todo: this should be replaced with a universal hash function
// (https://en.wikipedia.org/wiki/Universal_hashing)
fn hash(x: u32) -> u32 {
    u32::wrapping_add(u32::wrapping_mul(x, 1664525), 1013904223)
}

// In theory, this should give more random sampling than
// `sample_u32` but, in practice, KLEE's solver very, very
// strongly prefers to find the smallest values so
// all this does is randomize the order in which values are generated.
//
// If we want actually random values, then maybe we need
// to generate a series of random values that we want the bottom
// N bits of the hash function to evaluate to?
pub fn random_sample_u32(log_samples: usize, x: u32) -> u32 {
    let samples = 1 << log_samples;
    let h = hash(x) & (samples - 1);
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
