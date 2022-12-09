use sscanf::sscanf;
use std::io::{prelude::*, Cursor, stdin};
use std::collections::HashSet;

struct Rope {
    visited: HashSet<(isize, isize)>,
    knots: Vec<(isize, isize)>
}

impl Rope {
    fn parse_all<R: BufRead>(reader: R, num_knots: usize) -> Self {
        let mut rope = Self::new(num_knots);

        for line in reader.lines().filter_map(|line| line.ok()) {
            if let Ok(n) = sscanf!(line, "R {}", usize) {
                for _ in 0..n { rope.move_relative(1, 0) }
            } else if let Ok(n) = sscanf!(line, "L {}", usize) {
                for _ in 0..n { rope.move_relative(-1, 0) }
            } else if let Ok(n) = sscanf!(line, "U {}", usize) {
                for _ in 0..n { rope.move_relative(0, 1) }
            } else if let Ok(n) = sscanf!(line, "D {}", usize) {
                for _ in 0..n { rope.move_relative(0, -1) }
            } else {
                panic!("unrecognized line -- {}", line)
            }
        }

        rope
    }

    fn new(num_knots: usize) -> Self {
        Self {
            visited: HashSet::from_iter([(0, 0)].into_iter()),
            knots: vec![(0, 0)].repeat(num_knots),
        }
    }

    fn move_relative(&mut self, dx: isize, dy: isize) {
        if let Some(head) = self.knots.first_mut() {
            head.0 += dx;
            head.1 += dy;

            for i in 1..self.knots.len() {
                Self::adjust_knot(self.knots[i-1], &mut self.knots[i]);
            }

            self.visited.insert(*self.knots.last().unwrap());
        }
    }

    fn adjust_knot(head: (isize, isize), tail: &mut (isize, isize)) {
        let diff_0 = head.0 - tail.0;
        let diff_1 = head.1 - tail.1;

        if (diff_0.abs() >= 2 && diff_1.abs() >= 1) || (diff_0.abs() >= 1 && diff_1.abs() >= 2) {
            tail.0 += diff_0.signum();
            tail.1 += diff_1.signum();
        } else if diff_0.abs() >= 2 {
            tail.0 += diff_0.signum();
        } else if diff_1.abs() >= 2 {
            tail.1 += diff_1.signum();
        }
    }

    fn num_visited(&self) -> usize {
        self.visited.len()
    }
}

fn main() {
    let mut movement = String::new();

    if let Ok(_) = stdin().lock().read_to_string(&mut movement) {
        println!("{}", Rope::parse_all(Cursor::new(&movement), 2).num_visited());
        println!("{}", Rope::parse_all(Cursor::new(&movement), 10).num_visited());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"#;

    const LARGE_EXAMPLE: &str = r#"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"#;

    #[test]
    fn _01_example() {
        let rope = Rope::parse_all(Cursor::new(EXAMPLE), 2);

        assert_eq!(rope.num_visited(), 13);
    }

    #[test]
    fn _02_example() {
        let rope = Rope::parse_all(Cursor::new(EXAMPLE), 9);

        assert_eq!(rope.num_visited(), 1);
    }

    #[test]
    fn _02_large_example() {
        let rope = Rope::parse_all(Cursor::new(LARGE_EXAMPLE), 10);

        assert_eq!(rope.num_visited(), 36);
    }
}
