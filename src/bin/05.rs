use sscanf::sscanf;
use std::{io::{prelude::*, stdin}, collections::VecDeque};

#[derive(Clone)]
struct Crate {
    stack: VecDeque<char>
}

impl Crate {
    fn empty() -> Self {
        Self {
            stack: VecDeque::new()
        }
    }

    fn push_front(&mut self, ch: char) {
        self.stack.push_front(ch);
    }

    fn push_back(&mut self, ch: char) {
        self.stack.push_back(ch);
    }

    fn peek(&self) -> Option<char> {
        self.stack.back().cloned()
    }

    fn pop(&mut self) -> Option<char> {
        self.stack.pop_back()
    }
}

#[derive(Clone)]
struct Crates {
    crates: Vec<Crate>
}

enum CrateParseResult {
    Crates(Vec<char>),
    Label(Vec<usize>),
    None
}

impl Crates {
    fn parse<R: BufRead>(reader: &mut R) -> Self {
        let mut crates = vec! [];

        for line in reader.lines().filter_map(|line| line.ok()) {
            match Self::parse_line(line) {
                CrateParseResult::None => break,
                CrateParseResult::Label(_) => {
                    // pass
                },
                CrateParseResult::Crates(crates_row) => {
                    for (i, &bottom) in crates_row.iter().enumerate() {
                        if crates.len() <= i {
                            crates.resize(i + 1, Crate::empty());
                        }

                        if bottom != '\0' {
                            crates[i].push_front(bottom);
                        }
                    }
                }
            }
        }

        Self { crates }
    }

    fn parse_line(line: String) -> CrateParseResult {
        if line.trim().is_empty() {
            CrateParseResult::None
        } else if line.contains('[') {
            let mut parts = vec! [];

            for part in Self::triplets(line).iter() {
                if let Ok(ch) = sscanf!(part, "[{}]", char) {
                    parts.push(ch);
                } else {
                    parts.push('\0');
                }
            }

            CrateParseResult::Crates(parts)
        } else {
            let mut parts = vec! [];

            for part in Self::triplets(line).iter() {
                if let Ok(ch) = sscanf!(part, " {} ", usize) {
                    parts.push(ch);
                } else {
                    parts.push(0);
                }
            }

            CrateParseResult::Label(parts)
        }
    }

    fn triplets(mut line: String) -> Vec<String> {
        let mut parts = vec! [];

        while line.len() >= 3 {
            let remains = line.split_off(3);
            parts.push(line);
            line = remains;

            if line.len() > 0 {
                line = line.split_off(1);
            }
        }

        parts
    }

    fn move_to(&mut self, from: usize, to: usize) {
        if let Some(ch) = self.crates[from].pop() {
            self.crates[to].push_back(ch);
        }
    }

    fn move_multiple_to(&mut self, amount: usize, from: usize, to: usize) {
        let mut temp = (0..amount).filter_map(|_| self.crates[from].pop()).collect::<Vec<_>>();

        while let Some(c) = temp.pop() {
            self.crates[to].push_back(c);
        }
    }

    fn top(&self) -> Vec<char> {
        self.crates.iter().filter_map(|c| c.peek()).collect()
    }
}

#[derive(PartialEq, Debug)]
struct Rearrangement {
    amount: usize,
    from: usize,
    to: usize
}

impl Rearrangement {
    fn parse_all<R: BufRead>(reader: &mut R) -> Vec<Self> {
        reader.lines()
            .filter_map(|line| line.ok())
            .filter_map(|line| {
                if let Ok((amount, from, to)) = sscanf!(line, "move {} from {} to {}", usize, usize, usize) {
                    Some(Rearrangement { amount, from: from, to })
                } else {
                    None
                }
            })
            .collect()
    }

    fn from(&self) -> usize {
        self.from - 1
    }

    fn to(&self) -> usize {
        self.to - 1
    }
}

fn main() {
    let mut stdin = stdin().lock();
    let mut crates = Crates::parse(&mut stdin);
    let mut crates2 = crates.clone();
    let rearrangements = Rearrangement::parse_all(&mut stdin);

    for op in &rearrangements {
        for _ in 0..op.amount {
            crates.move_to(op.from(), op.to());
        }

        crates2.move_multiple_to(op.amount, op.from(), op.to());
    }

    println!("{}", crates.top().iter().collect::<String>());
    println!("{}", crates2.top().iter().collect::<String>());
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    const EXAMPLE: &str = r#"    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;

    #[test]
    fn _01_example() {
        let mut reader = Cursor::new(EXAMPLE);
        let mut crates = Crates::parse(&mut reader);
        let rearrangements = Rearrangement::parse_all(&mut reader);

        assert_eq!(crates.crates[0].stack, vec! ['Z', 'N']);
        assert_eq!(crates.crates[1].stack, vec! ['M', 'C', 'D']);
        assert_eq!(crates.crates[2].stack, vec! ['P']);
        assert_eq!(rearrangements[0], Rearrangement { amount: 1, from: 2, to: 1 });
        assert_eq!(rearrangements[1], Rearrangement { amount: 3, from: 1, to: 3 });
        assert_eq!(rearrangements[2], Rearrangement { amount: 2, from: 2, to: 1 });
        assert_eq!(rearrangements[3], Rearrangement { amount: 1, from: 1, to: 2 });

        for op in &rearrangements {
            for _ in 0..op.amount {
                crates.move_to(op.from(), op.to());
            }
        }

        assert_eq!(crates.top(), vec! ['C', 'M', 'Z']);
    }

    #[test]
    fn _02_example() {
        let mut reader = Cursor::new(EXAMPLE);
        let mut crates = Crates::parse(&mut reader);
        let rearrangements = Rearrangement::parse_all(&mut reader);

        for op in &rearrangements {
            crates.move_multiple_to(op.amount, op.from(), op.to());
        }

        assert_eq!(crates.top(), vec! ['M', 'C', 'D']);
    }
}
