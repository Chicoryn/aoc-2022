use std::io::{prelude::*, stdin};
use std::cmp::Ordering;

#[derive(Clone, PartialEq, Eq)]
enum Packet {
    Array(Vec<Packet>),
    Int(i64),
}

impl Packet {
    fn dividers() -> Vec<Self> {
        vec! [
            Packet::Array(vec! [Packet::Array(vec! [Packet::Int(2)])]),
            Packet::Array(vec! [Packet::Array(vec! [Packet::Int(6)])]),
        ]
    }

    fn split_chunks(packets: &[Packet]) -> Vec<(Self, Self)> {
        packets.chunks(2)
            .map(|chunk| (chunk[0].clone(), chunk[1].clone()))
            .collect()
    }

    fn parse_all<R: BufRead>(reader: R) -> Vec<Self> {
        reader.lines()
            .filter_map(|line| line.ok().filter(|line| !line.is_empty()))
            .map(|line| Self::parse(&line))
            .collect::<Vec<_>>()
    }

    fn parse(line: &str) -> Self {
        Packet::from(&serde_json::from_str::<serde_json::Value>(line).unwrap())
    }

    fn compare_to(&self, right: &Self) -> Ordering {
        match (self, right) {
            (left @ Self::Int(_), right @ Self::Array(_)) => Self::Array(vec! [left.clone()]).compare_to(right),
            (left @ Self::Array(_), right @ Self::Int(_)) => left.compare_to(&Self::Array(vec! [right.clone()])),
            (Self::Int(left), Self::Int(right)) => left.partial_cmp(&right).unwrap(),
            (Self::Array(left), Self::Array(right)) => {
                let n = left.len().min(right.len());

                (0..n).fold(Ordering::Equal, |so_far, i| {
                    so_far.then_with(|| left[i].compare_to(&right[i]))
                }).then_with(|| left.len().partial_cmp(&right.len()).unwrap())
            },
        }
    }
}

impl From<&serde_json::Value> for Packet {
    fn from(other: &serde_json::Value) -> Self {
        match other {
            serde_json::Value::Number(n) => Packet::Int(n.as_i64().unwrap()),
            serde_json::Value::Array(ns) => Self::Array(ns.iter().map(|x| Self::from(x)).collect()),
            _ => panic!()
        }
    }
}

fn main() {
    let stdin = stdin().lock();
    let mut packets = Packet::parse_all(stdin);
    let chunks = Packet::split_chunks(&packets);
    let dividers = Packet::dividers();
    packets.extend_from_slice(&dividers);
    packets.sort_by(|a, b| a.compare_to(b));

    println!("{}", chunks.iter().enumerate().map(|(i, pair)| if pair.0.compare_to(&pair.1) == Ordering::Less { i + 1 } else { 0 }).sum::<usize>());
    println!("{}", packets.iter().enumerate().fold(1, |so_far, (i, packet)| if dividers.contains(packet) { so_far * (i + 1) } else { so_far }));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    const EXAMPLE: &str = r#"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"#;

    #[test]
    fn _01_example() {
        let packets = Packet::parse_all(Cursor::new(EXAMPLE));
        let chunks = Packet::split_chunks(&packets);

        assert_eq!(chunks.len(), 8);
        assert_eq!(chunks[0].0.compare_to(&chunks[0].1), Ordering::Less);
        assert_eq!(chunks[1].0.compare_to(&chunks[1].1), Ordering::Less);
        assert_eq!(chunks[2].0.compare_to(&chunks[2].1), Ordering::Greater);
        assert_eq!(chunks[3].0.compare_to(&chunks[3].1), Ordering::Less);
        assert_eq!(chunks[4].0.compare_to(&chunks[4].1), Ordering::Greater);
        assert_eq!(chunks[5].0.compare_to(&chunks[5].1), Ordering::Less);
        assert_eq!(chunks[6].0.compare_to(&chunks[6].1), Ordering::Greater);
        assert_eq!(chunks[7].0.compare_to(&chunks[7].1), Ordering::Greater);
        assert_eq!(chunks.iter().enumerate().map(|(i, pair)| if pair.0.compare_to(&pair.1) == Ordering::Less { i + 1 } else { 0 }).sum::<usize>(), 13);
    }

    #[test]
    fn _02_example() {
        let mut packets = Packet::parse_all(Cursor::new(EXAMPLE));
        let dividers = Packet::dividers();
        packets.extend_from_slice(&dividers);
        packets.sort_by(|a, b| a.compare_to(b));

        assert_eq!(packets.iter().position(|packet| packet == &dividers[0]).map(|i| i + 1), Some(10));
        assert_eq!(packets.iter().position(|packet| packet == &dividers[1]).map(|i| i + 1), Some(14));
        assert_eq!(packets.iter().enumerate().fold(1, |so_far, (i, packet)| if dividers.contains(packet) { so_far * (i + 1) } else { so_far }), 140);
    }
}
