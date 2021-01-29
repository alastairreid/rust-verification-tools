use std::collections::HashMap;
use std::io;
use std::io::prelude::*;

fn main() {
    let n = io::stdin()
        .lock()
        .lines()
        .filter(|x| is_nice(x.as_ref().unwrap().as_str()))
        .count();
    println!("Nice strings = {}", n)
}

/// Contains at least 3 vowels
///
/// Note ambiguity: do the vowels have to be distinct?
fn rule1(x: &str) -> bool {
    let n = x.chars()
        .filter(|&c| "aeiou".contains(|d| c == d))
        .count();
    n >= 3
}

/// Contains at least one repeated character
fn rule2(x: &str) -> bool {
    let mut y = x.chars();
    y.next(); // skip first character
    x.chars()
        .zip(y)
        .position(|(c, d)| c == d)
        .is_some()
}

/// Does not contain naughty substrings
fn rule3(x: &str) -> bool {
    // todo: could rewrite using bad_strs.all()
    for s in ["ab", "cd", "pq", "xy"].iter() {
        if x.contains(s) {
            return false
        }
    }
    return true
}

fn is_nice(x: &str) -> bool {
    rule1(x) & rule2(x) & rule3(x)
}

mod rule_tests {
    #[cfg(not(verify))]
    use proptest::prelude::*;
    #[cfg(verify)]
    use propverify::prelude::*;
    use super::*;

    #[test]
    fn test_rule1() {
        assert!(rule1("aei"));
        assert!(rule1("xazegov"));
        assert!(rule1("aeiouaeiouaeiou"));
    }

    // proptest! {
    //     #[test]
    //     fn prop_rule1(x: &str, c: char) {
    //         if rule1(x) {
    //             prop_assert!(rule1([x, c].concat()))
    //         } else {
    //             if rule1([x, c].concat()) {
    //                 prop_assert!("aeiou".contains(|d| c == d))
    //             }
    //         }
    //     }
    // }

    #[test]
    fn test_rule2() {
        assert!(rule2("xx"));
        assert!(rule2("abcdde"));
        assert!(rule2("aabbccdd"));
    }

    #[test]
    fn test_rule3() {
        assert!(!rule3("aabb"));
        assert!(!rule3("xxcdxx"));
        assert!(!rule3("apqrs"));
        assert!(!rule3("xxyy"));
        assert!(rule3("efgh"));
    }

    #[test]
    fn test_nice() {
        assert!(is_nice("ugknbfddgicrmopn"));
        assert!(is_nice("aaa"));
        assert!(!is_nice("jchzalrnumimnmhp"));
        assert!(!is_nice("haegwjzuvuyypxyu"));
        assert!(!is_nice("dvszwmarrgswjxmb"));
    }
}

/// It contains a pair of any two letters that appears at least twice in the string
/// without overlapping, like xyxy (xy) or aabcdefgaa (aa), but not like aaa (aa, but it overlaps).
///
/// (I think this means adjacent letters)
fn rule4(x: &str) -> bool {
    let mut repeats = HashMap::new();
    let mut y = x.chars();
    y.next();
    for (i, (c, d)) in x.chars().zip(y).enumerate() {
        if let Some(j) = repeats.get(&(c,d)) {
            if i-j >= 2 {
                return true
            }
        }
        repeats.insert((c, d), i);
    }
    false
}


/// It contains at least one letter which repeats with exactly one letter between them,
/// like xyx, abcdefeghi (efe), or even aaa.
fn rule5(x: &str) -> bool {
    let mut y = x.chars();
    y.next(); // skip two characters
    y.next();
    x.chars()
        .zip(y)
        .position(|(c, d)| c == d)
        .is_some()
}

fn is_nice2(x: &str) -> bool {
    rule4(x) & rule5(x)
}


mod rule_tests2 {
    use super::*;

    #[test]
    fn test_rule4() {
        assert!(rule4("xyxy"));
        assert!(rule4("aabcdefgaa"));
        assert!(!rule4("aaa"));
    }

    #[test]
    fn test_rule5() {
        assert!(rule5("xyx"));
        assert!(rule5("abcdefeghi"));
        assert!(rule5("aaa"));
        assert!(!rule5("abba"));
    }

    #[test]
    fn test_nice() {
        assert!(is_nice2("qjhvhtzxzqqjkmpb"));
        assert!(is_nice2("xxyxx"));
        assert!(!is_nice2("uurcxstgmygtbstg"));
        assert!(!is_nice2("ieodomkazucvgmuy"));
    }
}
