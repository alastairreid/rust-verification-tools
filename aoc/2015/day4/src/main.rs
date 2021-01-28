use md5;

fn main() {
    println!("Hello, world!");
    println!("{:?}", hash("abcdef", "609043"));
    println!("{:?}", is_coin("abcdef", "609043", 5));
    println!("{:?}", hash("pqrstuv", "1048970"));
    println!("{:?}", is_coin("pqrstuv", "1048970", 5));
    println!("{}", mine("abcdef", 5));
    println!("{}", mine("pqrstuv", 5));
    println!("{}", mine("bgvyzdsv", 5));
    println!("{}", mine("bgvyzdsv", 6));
}

fn mine(key: &str, n: usize) -> u64 {
    for i in 1u64.. {
        if 0 == i % 1000 {
            print!(".");
        }
        let suffix = format!("{}", i);
        if is_coin(key, &suffix, n) {
            return i;
        }
    }
    panic!("Failed mining")
}

fn is_coin(key: &str, suffix: &str, n: usize) -> bool {
    let h = hash(key, suffix);
    let s = format!("{:#032x}", h);
    let s = s.as_str();
    &s[0..n] == "0".repeat(n)
}

fn hash(key: &str, suffix: &str) -> md5::Digest {
    let m = [key.as_bytes(), suffix.as_bytes()].concat();
    md5::compute(m.clone())
}

mod test {
    use super::*;

    #[test]
    fn hash1() {
        assert_eq!(hash("abcdef", "609043")[0], 0u8);
    }

    #[test]
    fn hash2() {
        assert_eq!(hash("pqrstuv", "1048970")[0], 0u8);
    }

    #[test]
    fn example1() {
        assert_eq!(mine("abcdef", 5), 609043)
    }

    #[test]
    fn example2() {
        assert_eq!(mine("pqrstuv", 5), 1048970)
    }

    // no obvious way to use proptest to test this
}
