use regex::Regex;
use std::io;
use std::io::prelude::*;

fn main() {
    let mut total = 0;
    let re = Regex::new(r"(\d+)x(\d+)x(\d+)").unwrap();
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let cap = re.captures(line.as_str()).unwrap();
        let caps: Vec<Result<u32, std::num::ParseIntError>> = cap.iter().map(|i| i.unwrap().as_str().parse::<u32>()).collect();
        match caps[1..] {
            [Ok(x), Ok(y), Ok(z)] => {
                let w = wrapping(x, y, z);
                total += w;
                println!("Wrapping for {}x{}x{} = {}", x, y, z, w);
            },
            _ => {
                panic!("Malformed input {:?}", cap);
            }
        }
    }
    println!("Total wrapping paper required = {}", total)
}

fn wrapping(x: u32, y: u32, z: u32) -> u32 {
    // todo: could sort [x,y,z] first:
    //   smallest side is product of two smallest edges
    let sides = [x * y, x * z, y * z];
    let smallest = *sides.iter().min().unwrap();
    let area = 2 * sides.iter().sum::<u32>();
    area + smallest
}

mod test_part1 {
    #[cfg(not(verify))]
    use proptest::prelude::*;
    #[cfg(verify)]
    use propverify::prelude::*;

    use super::wrapping;

    #[test]
    /// Example from the problem statement
    fn example_1() {
        assert_eq!(wrapping(2, 3, 4), 58)
    }

    #[test]
    /// Example from the problem statement
    fn example_2() {
        assert_eq!(wrapping(1, 1, 10), 43)
    }

    proptest! {
        #[test]
        fn zero(z: u32) {
            prop_assert_eq!(wrapping(z, 0, 0), 0)
        }
    }

    proptest! {
        #[test]
        // Note use of restricted range to avoid arithmetic overflow
        fn double(x in 0..1000u32, y in 0..1000u32, z in 0..1000u32) {
            prop_assert_eq!(4 * wrapping(x, y, z), wrapping(2*x, 2*y, 2*z))
        }
    }

    proptest! {
        #[test]
        /// The order of the different sides does not affect the result
        ///
        /// Note use of restricted range to avoid arithmetic overflow
        fn reorder(x in 0..1000u32, y in 0..1000u32, z in 0..1000u32) {
            prop_assert_eq!(wrapping(x, y, z), wrapping(x, z, y));
            prop_assert_eq!(wrapping(x, y, z), wrapping(y, x, z));
            prop_assert_eq!(wrapping(x, y, z), wrapping(y, z, x));
            prop_assert_eq!(wrapping(x, y, z), wrapping(z, x, y));
            prop_assert_eq!(wrapping(x, y, z), wrapping(z, y, x));
        }
    }

    // Can't express in proptest
    // proptest!{
    //     #[test]
    //     // Note use of restricted range to avoid arithmetic overflow
    //     fn bigger(x1 in 0..1000u32, y1 in 0..1000u32, z1 in 0..1000u32,
    //               x2 in x1..1000u32, y2 in y1..1000u32, z2 in z1..1000u32,
    //               ) {
    //         prop_assert!(wrapping(x1, y1, z1) <= wrapping(x2, y2, z2))
    //     }
    // }
}
