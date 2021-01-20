#[allow(unused_imports)]
use std;

fn main() {
    for dirs in std::env::args() {
        println!("Santa {} = {}", dirs, santa(&dirs));
        println!(
            "Santa goes underground {} at step {:?}",
            dirs,
            underground_santa(&dirs)
        )
    }
}

// todo: first attempt was wrong: used a usize for result
fn santa(dirs: &str) -> isize {
    let mut count = 0;
    for c in dirs.chars() {
        if c == '(' {
            count += 1;
        } else if c == ')' {
            count -= 1;
        } else {
            // first attempt: panic!("Malformed string");
        }
    }
    count
}

mod test_part1 {
    #[cfg(not(verify))]
    use proptest::prelude::*;
    #[cfg(verify)]
    use propverify::prelude::*;

    use super::santa;

    use std::convert::TryFrom;

    #[test]
    fn empty() {
        assert_eq!(santa(""), 0)
    }

    fn santa_onechar(x: char) -> isize {
        if x == '(' {
            1
        } else if x == ')' {
            -1
        } else {
            0
        }
    }

    proptest! {
        #[test]
        fn singleton(x: char) {
            let r = santa_onechar(x);
            prop_assert_eq!(santa(&x.to_string()), r); // nope
        }
    }

    proptest! {
        #[test]
        fn append(x: String, y: String) {
            prop_assert_eq!(santa(x.as_str()) + santa(y.as_str()), santa((x + &y).as_str()))
        }
    }

    proptest! {
        #[test]
        fn up(x in r"\(*") {
            let r = isize::try_from(x.len())?;
            prop_assert_eq!(santa(x.as_str()), r)
        }
    }

    proptest! {
        #[test]
        fn down(x in r"\)*") {
            let r = isize::try_from(x.len())?;
            prop_assert_eq!(santa(x.as_str()), -r)
        }
    }

    proptest! {
        #[test]
        fn none(x in r"[^()]*") {
            prop_assert_eq!(santa(x.as_str()), 0)
        }
    }

    proptest! {
        #[test]
        fn filtered(x: String) {
            let ups = isize::try_from(x.chars().filter(|c| *c == '(').count())?;
            let downs = isize::try_from(x.chars().filter(|c| *c == ')').count())?;
            prop_assert_eq!(santa(x.as_str()), ups - downs)
        }
    }

    fn santa_spec(dirs: &str) -> isize {
        let ups: isize = isize::try_from(dirs.chars().filter(|c| *c == '(').count()).unwrap();
        let downs: isize = isize::try_from(dirs.chars().filter(|c| *c == ')').count()).unwrap();
        ups - downs
    }

    proptest! {
        #[test]
        fn filtered2(x: String) {
            prop_assert_eq!(santa(x.as_str()), santa_spec(x.as_str()))
        }
    }
}

// part two of the puzzle
fn underground_santa(dirs: &str) -> Option<usize> {
    let mut count = 0;
    for (i, c) in dirs.chars().enumerate() {
        // todo: I don't like this repetition from previous solution much
        // todo: should we consider a solution that answers both puzzles
        // in a single pass?
        if c == '(' {
            count += 1;
        } else if c == ')' {
            count -= 1;
        } else {
        }
        if count < 0 {
            return Some(i + 1);
        }
    }
    None
}

mod test_part2 {
    #[cfg(not(verify))]
    use proptest::prelude::*;
    #[cfg(verify)]
    use propverify::prelude::*;

    use super::{santa, underground_santa};

    #[test]
    fn empty() {
        assert_eq!(underground_santa(""), None) // 0 indicates failure
    }

    proptest! {
        /// Test that adding one character either
        /// - returns previous result if already found basement; or
        /// - reaches the basement if we are at floor zero and we go down; or
        /// - still haven't reached the basement
        #[test]
        fn push(x: String, c: char) {
            let r = if let Some(rx) = underground_santa(x.as_str()) {
                Some(rx)
            } else if santa(x.as_str()) == 0 && c == ')' {
                // Incorrect first attempt used len (number of bytes) instead of
                // number of characters
                // Some(x.len() + 1)
                Some(x.chars().count() + 1)
            } else {
                None
            };
            let mut x = x;
            x.push(c);
            prop_assert_eq!(underground_santa(x.as_str()), r)
        }
    }

    proptest! {
        /// Test that if we have already reached the basement in `x`,
        /// then we stop digging no matter what is in `y`.
        #[test]
        fn append(x: String, y: String) {
            if underground_santa(x.as_str()).is_some() {
                prop_assert_eq!(
                    underground_santa(x.as_str()),
                    underground_santa((x + &y).as_str()))
            }
        }
    }

    proptest! {
        /// Various lower and upper bounds on the result
        #[test]
        fn length(x: String) {
            match underground_santa(x.as_str()) {
                Some(rx) => {
                    prop_assert!(1  <= rx);
                    prop_assert!(rx <= x.len());
                    prop_assert!(rx <= x.chars().count())
                },
                _ =>
                    ()
            }
        }
    }

    proptest! {
        /// We must be going down when we hit the basement
        #[test]
        fn going_down(x: String) {
            match underground_santa(x.as_str()) {
                Some(rx) => {
                    prop_assert_eq!(x.chars().nth(rx-1).unwrap(), ')')
                },
                _ => {
                }
            }
        }
    }

    proptest! {
        /// The value must be -1 when we hit the basement
        #[test]
        fn basement1(x: String) {
            match underground_santa(x.as_str()) {
                Some(rx) => {
                    prop_assert_eq!(Some(rx), underground_santa(x.chars().take(rx).collect::<String>().as_str()))
                },
                _ => {
                }
            }
        }
    }
}
