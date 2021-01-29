use itertools::Itertools;
use std::fs;
use std::io;


fn main() -> io::Result<()> {
    let directions = fs::read_to_string("input1.txt")?;
    println!("Santa visited {} unique houses", num_visited(directions.as_str()));
    Ok(())
}

fn delta(c: char) -> (i32, i32) {
    if c == '<' {
        (-1, 0)
    } else if c == '>' {
        (1, 0)
    } else if c == '^' {
        (0, 1)
    } else if c == 'v' {
        (0, -1)
    } else {
        (0, 0)
    }
}

fn num_visited(dirs: &str) -> usize {
    let start: (i32, i32) = (0, 0);
    let houses =
        std::iter::once(start)
        .chain(
            dirs
            .chars()
            // not necessary
            // .filter(|c| "<>^v".contains(*c))
            .scan(start, |pos, d| {
                let (dx, dy) = delta(d);
                *pos = (pos.0 + dx, pos.1 + dy);
                Some(*pos)
            })
        );
    let houses = houses.unique();
    let houses = houses.count();
    houses
}

mod tests_part1 {
    use super::num_visited;

    #[test]
    fn examples() {
        assert_eq!(2, num_visited(">"));
        assert_eq!(4, num_visited("^>v<"));
        assert_eq!(2, num_visited("^v^v^v^v^v"));
    }
}