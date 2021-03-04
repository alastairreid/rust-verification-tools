////////////////////////////////////////////////////////////////
// Several variations on a theme to test failing variants
////////////////////////////////////////////////////////////////

use crate as verifier;

use crate::assert;
use regex::Regex;

#[cfg_attr(not(feature = "verifier-crux"), test)]
#[cfg_attr(feature = "verifier-crux", crux_test)]
fn t0() {
    let a : u32 = verifier::AbstractValue::abstract_value();
    let b : u32 = verifier::AbstractValue::abstract_value();
    verifier::assume(4 <= a && a <= 7);
    verifier::assume(5 <= b && b <= 8);

    #[cfg(not(any(feature = "verifier-crux", feature = "verifier-seahorn")))]
    if verifier::is_replay() { eprintln!("Test values: a = {}, b = {}", a, b) }

    let r = a*b;
    assert!(20 <= r && r <= 56);
}

#[cfg_attr(not(feature = "verifier-crux"), test)]
#[cfg_attr(feature = "verifier-crux", crux_test)]
fn t1() {
    let a : u32 = verifier::AbstractValue::abstract_value();
    let b : u32 = verifier::AbstractValue::abstract_value();
    verifier::assume(4 <= a && a <= 7);
    verifier::assume(5 <= b && b <= 8);
    let r = a*b;
    assert!(20 <= r && r <= 56);
}

#[cfg_attr(not(feature = "verifier-crux"), test)]
#[cfg_attr(feature = "verifier-crux", crux_test)]
fn t2() {
    #[cfg(not(feature = "verifier-crux"))]
    verifier::expect(Some("multiply with overflow"));

    let a : u32 = verifier::AbstractValue::abstract_value();
    let b : u32 = verifier::AbstractValue::abstract_value();
    let r = a*b;
    verifier::assume(4 <= a && a <= 7);
    verifier::assume(5 <= b && b <= 8);
    assert!(20 <= r && r <= 56);
}

#[cfg_attr(not(feature = "verifier-crux"), test)]
#[cfg_attr(feature = "verifier-crux", crux_test)]
fn t3() {
    #[cfg(not(feature = "verifier-crux"))]
    verifier::expect(Some("assertion failed"));

    let a : u32 = verifier::AbstractValue::abstract_value();
    let b : u32 = verifier::AbstractValue::abstract_value();
    verifier::assume(4 <= a && a <= 7);
    verifier::assume(5 <= b && b <= 8);
    let r = a*b;
    assert!(20 <= r && r < 56);
}

#[cfg_attr(not(feature = "verifier-crux"), test)]
#[cfg_attr(feature = "verifier-crux", crux_test)]
fn t4() {
    #[cfg(not(feature = "verifier-crux"))]
    verifier::expect(None);

    let a : u32 = verifier::AbstractValue::abstract_value();
    let b : u32 = verifier::AbstractValue::abstract_value();
    verifier::assume(4 <= a && a <= 7);
    verifier::assume(5 <= b && b <= 8);
    let r = a*b;
    assert!(20 <= r && r < 56);
}

#[cfg_attr(not(feature = "verifier-crux"), test)]
#[cfg_attr(feature = "verifier-crux", crux_test)]
fn t5() {
    let a : u32 = verifier::AbstractValue::abstract_value();
    let b : u32 = verifier::AbstractValue::abstract_value();
    verifier::assume(a <= 1000000); // avoid overflow
    verifier::assume(b <= 1000000);
    verifier::assert_eq!(a + b, b + a);
    verifier::assert_ne!(a, a+1);
}

#[cfg_attr(not(feature = "verifier-crux"), test)]
#[cfg_attr(feature = "verifier-crux", crux_test)]
fn bytes() {
    let a = verifier::verifier_nondet_bytes(8);
    for i in a.iter() {
        verifier::assume(*i == 42);
    }
    if verifier::is_replay() {
        println!("{:?}", a);
    }
    verifier::assert_eq!(a.len(), 8);
    verifier::assert_ne!(a[2], 0u8);
    verifier::assert_eq!(a[3], 42u8);
}

#[cfg_attr(not(feature = "verifier-crux"), test)]
#[cfg_attr(feature = "verifier-crux", crux_test)]
fn concretize_exhaustive() {
    let a: u32 = verifier::AbstractValue::abstract_value();
    verifier::assume(4 <= a && a <= 7);
    let a = verifier::concretize_u32(a);
    println!("a = {}", a)
}

// hmmm, I thought I had tested this - but it doesn't even
// compile
//
// #[cfg_attr(not(feature = "verifier-crux"), test)]
// #[cfg_attr(feature = "verifier-crux", crux_test)]
// fn varied_arrays() {
//     let n: usize = verifier::AbstractValue::abstract_value();
//     verifier::assume(4 <= n && n <= 7);
//     let n = verifier::concretize_usize(n);
//     let a = [42; n];
//     println!("n = {}", n);
//     verifier::assert_eq!(a.len(), n);
//     verifier::assert_ne!(a[2], 0u8);
//     verifier::assert_eq!(a[3], 42u8);
// }

#[cfg_attr(not(feature = "verifier-crux"), test)]
#[cfg_attr(feature = "verifier-crux", crux_test)]
fn varied_bytes() {
    let n: usize = verifier::AbstractValue::abstract_value();
    verifier::assume(4 <= n && n <= 7);
    // Comment out the following line to see KLEE complain about allocating an
    // object whose length is symbolic.
    // (If you run KLEE without the flag "--exit-on-error", an error is reported
    // but execution continues with a single concrete value.)
    //
    //   KLEE: ERROR: (location information missing) concretized symbolic size
    //   ...
    //   Stack:
    //       #000014533 in __rdl_alloc (, =1)
    //       #100013891 in __rust_alloc (, =1)
    //       ...
    //       #900496959 in _ZN24verification_annotations5tests12varied_bytes17hc04099c3fe734351E () at src/tests.rs:119
    //   Info:
    //     size expr: (Extract w64 0 (ZExt w128 (ReadLSB w64 0 unnamed)))
    //     concretization : 4
    //     unbound example: 5
    //
    let n = verifier::concretize_usize(n);
    let a = verifier::verifier_nondet_bytes(n);
    for i in a.iter() {
        verifier::assume(*i == 42);
    }
    if verifier::is_replay() {
        println!("{:?}", a);
    }
    println!("n = {}", n);
    verifier::assert_eq!(a.len(), n);
    verifier::assert_ne!(a[2], 0u8);
    verifier::assert_eq!(a[3], 42u8);
}

/// Check for colliding assignments
///
/// (Not very interesting  - I was confused about this test)
#[cfg_attr(not(feature = "verifier-crux"), test)]
#[cfg_attr(feature = "verifier-crux", crux_test)]
fn collide() {
    let mut a = [0u32; 100];

    let i: usize = verifier::AbstractValue::abstract_value();
    verifier::assume(i < 100);

    let j: usize = verifier::AbstractValue::abstract_value();
    verifier::assume(j < 100);

    a[i] = 42;

    // verifier::concretize_usize(j);
    verifier::assert_eq!(a[j], if i == j { 42 } else { 0 });
}


/// Copy the first 'n' elements of an array to another
///
/// This is a scaling test to explore path explosions
#[cfg_attr(not(feature = "verifier-crux"), test)]
#[cfg_attr(feature = "verifier-crux", crux_test)]
fn copy() {
    let mut a = [0u32; 100];
    let mut b = [1u32; 100];

    // choose how much data to copy
    let n: usize = verifier::AbstractValue::abstract_value();
    verifier::assume(n <= a.len());

    if false {
        for i in 0..n {
            b[i] = a[i];
        }
    } else if false {
        verifier::concretize_usize(n);
        for i in 0..n {
            b[i] = a[i];
        }
    } else if false {
        verifier::coherent!{{
            verifier::concretize_usize(n);
            for i in 0..n {
                b[i] = a[i];
            }
        }}
    } else {
        verifier::sample_u32(16, n as u32) as usize;
        // verifier::random_sample_u32(4, n as u32) as usize;
        println!("n = {}", n);
        for i in 0..n {
            b[i] = a[i];
        }
    }

    let i: usize = verifier::AbstractValue::abstract_value();
    verifier::assume(i < 100);
    verifier::assume(i < b.len());
    verifier::assert_eq!(b[i], if i < n { 0 } else { 1 })
}

#[cfg_attr(not(feature = "verifier-crux"), test)]
#[cfg_attr(feature = "verifier-crux", crux_test)]
fn cstring() {
    let a = verifier::verifier_nondet_cstring(100);

    if verifier::is_replay() {
        println!("{:?}", a);
    }

    // force string to be plain ASCII - to keep things simple

    // three variants of this code to deal with path explosion
    // in different ways
    if false {
        // no merging
        for i in a.as_bytes() {
            verifier::assume(i.is_ascii_alphabetic());
        }
    } else if false {
        // merging all loop iterations
        verifier::coherent! {{
            // force string to be plain ASCII - to keep things simple
            for i in a.as_bytes() {
                verifier::assume(i.is_ascii_alphabetic());
            }
        }}
    } else {
        // merging body of loop only (since number of iterations is constant)
        for i in a.as_bytes() {
            verifier::coherent! {{
                verifier::assume(i.is_ascii_alphabetic());
            }}
        }
    }

    for i in a.as_bytes() {
        verifier::assert!(i.is_ascii());
        // this assertion would fail
        // verifier::assert!(i.is_ascii_digit());
    }
}

#[cfg_attr(not(feature = "verifier-crux"), test)]
#[cfg_attr(feature = "verifier-crux", crux_test)]
fn string_ok() {
    let a = verifier::verifier_nondet_ascii_string(6);

    if verifier::is_replay() {
        println!("{:?}", a);
    }

    // force string to be a legal int
    for i in a.as_bytes().into_iter() {
        verifier::assume(('0'..='3').contains(&(*i as char)))
    }

    let i: u32 = a.parse().unwrap();
    verifier::assert!(i <= 333_333);
}

#[cfg_attr(not(feature = "verifier-crux"), test)]
#[cfg_attr(feature = "verifier-crux", crux_test)]
fn string_should_fail() {
    verifier::expect(Some("assertion failed"));
    let a = verifier::verifier_nondet_ascii_string(6);

    if verifier::is_replay() {
        println!("{:?}", a);
    }

    // force string to be a legal int
    for i in a.as_bytes().into_iter() {
        verifier::assume(('0'..='3').contains(&(*i as char)))
    }

    let i: u32 = a.parse().unwrap();
    verifier::assert!(i <= 222_222);
}

#[cfg_attr(not(feature = "verifier-crux"), test)]
#[cfg_attr(feature = "verifier-crux", crux_test)]
fn regex_ok() {
    let a = verifier::verifier_nondet_ascii_string(2);

    if verifier::is_replay() {
        println!("Value a = {:?}", a);
    }

    verifier::assume(Regex::new(r"[0-1]{2}").unwrap().is_match(&a));

    let i: u32 = a.parse().unwrap();
    verifier::assert!(i <= 11);
}

#[cfg_attr(not(feature = "verifier-crux"), test)]
#[cfg_attr(feature = "verifier-crux", crux_test)]
fn regex_should_fail() {
    verifier::expect(Some("assertion failed"));
    let a = verifier::verifier_nondet_ascii_string(2);

    if verifier::is_replay() {
        println!("Value a = {:?}", a);
    }

    verifier::assume(Regex::new(r"[0-1]{2}").unwrap().is_match(&a));

    let i: u32 = a.parse().unwrap();
    verifier::assert!(i < 11);
}

////////////////////////////////////////////////////////////////
// End
////////////////////////////////////////////////////////////////
