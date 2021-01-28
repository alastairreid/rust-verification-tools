use md5;

fn main() {
    println!("Hello, world!");
    println!("{}", mine("abcdef"));
    println!("{}", mine("pqrstuv"));
}

fn mine(key: &str) -> u64 {
    let key: &[u8] = key.as_bytes();
    for i in 1u64.. {
        if 0 == i % 1000 {
            print!(".");
        }
        let suffix = format!("{}", i);
        let m = [key, suffix.as_bytes()].concat();
        let h: [u8; 16] = md5::compute(m.clone()).0;
        let is_top = h[15] == 0u8 && h[14] == 0u8 && h[13] < 16;
        if is_top {
            return i;
        }
    }
    panic!("Failed mining")
}

mod test {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(mine("abcdef"), 609043)
    }

    #[test]
    fn example2() {
        assert_eq!(mine("pqrstuv"), 1048970)
    }
}
