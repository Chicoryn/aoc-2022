use std::io::{prelude::*, stdin};
use sscanf::sscanf;

pub struct Elf {
    calories: Vec<usize>
}

impl Elf {
    pub fn empty() -> Self {
        Self { calories: vec! [] }
    }

    pub fn parse<R: BufRead>(reader: R) -> Vec<Self> {
        let mut elves = vec! [
            Elf::empty()
        ];

        for line in reader.lines() {
            let line = line.unwrap();

            if line.is_empty() {
                elves.push(Elf::empty());
            } else if let Ok(item) = sscanf!(line, "{}", usize) {
                elves.last_mut().unwrap().calories.push(item);
            }
        }

        elves
    }

    pub fn total(&self) -> usize {
        self.calories.iter().sum()
    }
}

fn top_3(elves: &[usize]) -> Vec<usize> {
    let n = elves.len();
    let mut ordered_elves = elves.to_vec();
    ordered_elves.sort_unstable();
    ordered_elves[(n - 3)..].to_vec()
}

fn main() {
    let stdin = stdin().lock();
    let elves = Elf::parse(stdin);
    let calories = elves.iter().map(|elf| elf.total()).collect::<Vec<_>>();

    println!("{}", calories.iter().max().unwrap());
    println!("{}", top_3(&calories).iter().sum::<usize>());
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn _01_example() {
        let example = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"#;

        let elves = Elf::parse(Cursor::new(&example));

        assert_eq!(elves.len(), 5);
        assert_eq!(elves[0].calories, vec! [1000, 2000, 3000]);
        assert_eq!(elves[0].total(), 6000);
        assert_eq!(elves[1].calories, vec! [4000]);
        assert_eq!(elves[1].total(), 4000);
        assert_eq!(elves[2].calories, vec! [5000, 6000]);
        assert_eq!(elves[2].total(), 11000);
        assert_eq!(elves[3].calories, vec! [7000, 8000, 9000]);
        assert_eq!(elves[3].total(), 24000);
        assert_eq!(elves[4].calories, vec! [10000]);
        assert_eq!(elves[4].total(), 10000);
    }
}
