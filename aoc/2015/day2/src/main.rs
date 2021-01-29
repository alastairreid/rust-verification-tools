use regex::Regex;
use std::io;
use std::io::prelude::*;

fn main() {
    let mut total_wrapping = 0;
    let mut total_ribbon = 0;
    let re = Regex::new(r"(\d+)x(\d+)x(\d+)").unwrap();
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let cap = re.captures(line.as_str()).unwrap();
        let caps: Vec<Result<u32, std::num::ParseIntError>> = cap
            .iter()
            .map(|i| i.unwrap().as_str().parse::<u32>())
            .collect();
        match caps[1..] {
            [Ok(x), Ok(y), Ok(z)] => {
                let w = wrapping(x, y, z);
                let r = ribbon(x, y, z);
                total_wrapping += w;
                total_ribbon += r;
                println!(
                    "For {}x{}x{} you need {} wrapping and {} ribbon",
                    x, y, z, w, r
                );
            }
            _ => {
                panic!("Malformed input {:?}", cap);
            }
        }
    }
    println!("Total wrapping paper required = {}", total_wrapping);
    println!("Total ribbon required = {}", total_ribbon);
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

    use std::cmp::{min, max};
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
        /// Parcels of zero size on two sides need no wrapping paper
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

    proptest!{
        #[test]
        /// Larger boxes need more wrapping paper
        ///
        /// Note use of restricted range to avoid arithmetic overflow
        fn bigger(x1 in 0..1000u32, y1 in 0..1000u32, z1 in 0..1000u32,
                  x2 in 0..1000u32, y2 in 0..1000u32, z2 in 0..1000u32,
                  ) {
            fn min_max(a: u32, b: u32) -> (u32, u32) {
                (min(a, b), max(a, b))
            }
            let (x1, x2) = min_max(x1, x2);
            let (y1, y2) = min_max(y1, y2);
            let (z1, z2) = min_max(z1, z2);
            prop_assert!(wrapping(x1, y1, z1) <= wrapping(x2, y2, z2))
        }
    }
}

fn ribbon(x: u32, y: u32, z: u32) -> u32 {
    let volume = x * y * z;
    let half_perimeters = [x + y, x + z, y + z];
    let smallest = *half_perimeters.iter().min().unwrap();
    let perimeter = 2 * smallest;
    perimeter + volume
}

mod test_part2 {
    #[cfg(not(verify))]
    use proptest::prelude::*;
    #[cfg(verify)]
    use propverify::prelude::*;

    use std::cmp::{min, max};
    use super::ribbon;

    #[test]
    /// Example from the problem statement
    fn example_1() {
        assert_eq!(ribbon(2, 3, 4), 34)
    }

    #[test]
    /// Example from the problem statement
    fn example_2() {
        assert_eq!(ribbon(1, 1, 10), 14)
    }

    proptest! {
        #[test]
        /// Parcels of zero size on two sides need no ribbon
        fn zero(z: u32) {
            prop_assert_eq!(ribbon(z, 0, 0), 0)
        }
    }

    proptest! {
        #[test]
        /// The order of the different sides does not affect the result
        ///
        /// Note use of restricted range to avoid arithmetic overflow
        fn reorder(x in 0..1000u32, y in 0..1000u32, z in 0..1000u32) {
            prop_assert_eq!(ribbon(x, y, z), ribbon(x, z, y));
            prop_assert_eq!(ribbon(x, y, z), ribbon(y, x, z));
            prop_assert_eq!(ribbon(x, y, z), ribbon(y, z, x));
            prop_assert_eq!(ribbon(x, y, z), ribbon(z, x, y));
            prop_assert_eq!(ribbon(x, y, z), ribbon(z, y, x));
        }
    }

    proptest!{
        #[test]
        /// Larger boxes need more wrapping paper
        ///
        /// Note use of restricted range to avoid arithmetic overflow
        fn bigger(x1 in 0..1000u32, y1 in 0..1000u32, z1 in 0..1000u32,
                  x2 in 0..1000u32, y2 in 0..1000u32, z2 in 0..1000u32,
                  ) {
            fn min_max(a: u32, b: u32) -> (u32, u32) {
                (min(a, b), max(a, b))
            }
            let (x1, x2) = min_max(x1, x2);
            let (y1, y2) = min_max(y1, y2);
            let (z1, z2) = min_max(z1, z2);
            prop_assert!(ribbon(x1, y1, z1) <= ribbon(x2, y2, z2))
        }
    }
}