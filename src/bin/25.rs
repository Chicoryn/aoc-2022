use std::{io::{prelude::*, stdin}, str::FromStr, fmt::Display, iter::Sum};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Snafu(i64);

impl Snafu {
    #[cfg(test)]
    fn new(n: i64) -> Self {
        Self(n)
    }
}

impl Sum<Snafu> for Snafu {
    fn sum<I: Iterator<Item=Snafu>>(iter: I) -> Snafu {
        Snafu(iter.map(|snafu| snafu.0).sum::<i64>())
    }
}

impl FromStr for Snafu {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.chars().fold(0, |n, ch| {
            5 * n + match ch {
                '2' => 2,
                '1' => 1,
                '0' => 0,
                '-' => -1,
                '=' => -2,
                _ => panic!(),
            }
        })))
    }
}

impl Display for Snafu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = vec! [];
        let mut n = self.0;

        while n != 0 {
            let to_add = match n % 5 {
                0 => 0,
                1 => 1,
                2 => 2,
                3 => -2,
                4 => -1,
                _ => unreachable!(),
            };

            n = (n - to_add) / 5;
            s.push(match to_add {
                2  => '2',
                1  => '1',
                0  => '0',
                -1 => '-',
                -2 => '=',
                _  => unreachable!(),
            });
        }

        write!(f, "{}", s.into_iter().rev().collect::<String>())
    }
}

fn main() {
    let stdin = stdin().lock();
    let numbers = stdin.lines()
        .filter_map(|line| line.ok().and_then(|line| line.parse::<Snafu>().ok()))
        .collect::<Vec<_>>();

    println!("{}", numbers.iter().cloned().sum::<Snafu>());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _01_example() {
        assert_eq!(Snafu::from_str("1=-0-2"), Ok(Snafu(1747)));
        assert_eq!(Snafu::from_str("12111"), Ok(Snafu(906)));
        assert_eq!(Snafu::from_str("2=0="), Ok(Snafu(198)));
        assert_eq!(Snafu::from_str("21"), Ok(Snafu(11)));
        assert_eq!(Snafu::from_str("2=01"), Ok(Snafu(201)));
        assert_eq!(Snafu::from_str("111"), Ok(Snafu(31)));
        assert_eq!(Snafu::from_str("20012"), Ok(Snafu(1257)));
        assert_eq!(Snafu::from_str("112"), Ok(Snafu(32)));
        assert_eq!(Snafu::from_str("1=-1="), Ok(Snafu(353)));
        assert_eq!(Snafu::from_str("1-12"), Ok(Snafu(107)));
        assert_eq!(Snafu::from_str("12"), Ok(Snafu(7)));
        assert_eq!(Snafu::from_str("1="), Ok(Snafu(3)));
        assert_eq!(Snafu::from_str("122"), Ok(Snafu(37)));
        assert_eq!(format!("{}", Snafu::new(4890)), "2=-1=0");
    }
}
