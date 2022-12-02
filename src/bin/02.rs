use std::io::{prelude::*, stdin};
use sscanf::sscanf;

struct Round {
    opponent: char,
    to_play: char
}

impl Round {
    pub fn parse(line: &str) -> Option<Self> {
        if let Ok((opponent, to_play)) = sscanf!(line, "{} {}", char, char) {
            Some(Round { opponent, to_play })
        } else {
            None
        }
    }

    pub fn parse_all<R: BufRead>(reader: R) -> Vec<Round> {
        let mut rounds = vec! [];

        for line in reader.lines().map(|line| line.unwrap()) {
            if let Some(round) = Round::parse(&line) {
                rounds.push(round);
            }
        }

        rounds
    }

    pub fn score(&self) -> usize {
        match (self.opponent, self.to_play) {
            ('A', 'X') => 1 + 3,
            ('A', 'Y') => 2 + 6,
            ('A', 'Z') => 3 + 0,

            ('B', 'X') => 1 + 0,
            ('B', 'Y') => 2 + 3,
            ('B', 'Z') => 3 + 6,

            ('C', 'X') => 1 + 6,
            ('C', 'Y') => 2 + 0,
            ('C', 'Z') => 3 + 3,

            _ => 0
        }
    }

    pub fn score2(&self) -> usize {
        match (self.opponent, self.to_play) {
            ('A', 'X') => 3 + 0,
            ('A', 'Y') => 1 + 3,
            ('A', 'Z') => 2 + 6,

            ('B', 'X') => 1 + 0,
            ('B', 'Y') => 2 + 3,
            ('B', 'Z') => 3 + 6,

            ('C', 'X') => 2 + 0,
            ('C', 'Y') => 3 + 3,
            ('C', 'Z') => 1 + 6,

            _ => 0
        }
    }
}

fn main() {
    let stdin = stdin().lock();
    let rounds = Round::parse_all(stdin);

    println!("{}", rounds.iter().map(|round| round.score()).sum::<usize>());
    println!("{}", rounds.iter().map(|round| round.score2()).sum::<usize>());
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn _01_example() {
        let example = r#"A Y
B X
C Z"#;
        let rounds = Round::parse_all(Cursor::new(&example));

        assert_eq!(rounds.iter().map(|round| round.score()).sum::<usize>(), 15);
    }

    #[test]
    fn _02_example() {
        let example = r#"A Y
B X
C Z"#;
        let rounds = Round::parse_all(Cursor::new(&example));

        assert_eq!(rounds.iter().map(|round| round.score2()).sum::<usize>(), 12);
    }
}
