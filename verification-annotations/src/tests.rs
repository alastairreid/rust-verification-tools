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
fn cstring() {
    let a = verifier::verifier_nondet_cstring(8);

    if verifier::is_replay() {
        println!("{:?}", a);
    }

    // force string to be plain ASCII - to keep things simple
    for i in a.as_bytes() {
        verifier::assume(i.is_ascii_alphabetic());
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
