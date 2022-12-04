use sscanf::sscanf;
use std::io::{prelude::*, stdin};

struct SectionAssignment {
    lower: usize,
    upper: usize
}

impl SectionAssignment {
    fn parse(line: &str) -> Option<Self> {
        if let Ok((lower, upper)) = sscanf!(line, "{}-{}", usize, usize) {
            assert!(lower <= upper);

            Some(Self { lower, upper })
        } else {
            None
        }
    }

    fn is_subset(&self, other: &Self) -> bool {
        self.lower >= other.lower && self.upper <= other.upper
    }

    fn overlap(&self, other: &Self) -> bool {
        (self.lower >= other.lower && self.lower <= other.upper)
            || (self.upper >= other.lower && self.upper <= other.upper)
    }
}

struct SectionAssignmentPair {
    assignments: Vec<SectionAssignment>
}

impl SectionAssignmentPair {
    fn parse_all<R: BufRead>(reader: R) -> Vec<Self> {
        reader.lines()
            .filter_map(|line| line.ok())
            .map(|line| SectionAssignmentPair::parse(&line.trim()))
            .collect::<Vec<_>>()
    }

    fn parse(line: &str) -> Self {
        Self {
            assignments: line.split(",").filter_map(|s| SectionAssignment::parse(s)).collect()
        }
    }

    fn any_pair_matches(&self, cmp: impl Fn(&SectionAssignment, &SectionAssignment) -> bool) -> bool {
        let n = self.assignments.len();

        for i in 0..n {
            for j in 0..n {
                if i != j && cmp(&self.assignments[i], &self.assignments[j]) {
                    return true;
                }
            }
        }

        false
    }

    fn has_redundant_assignment(&self) -> bool {
        self.any_pair_matches(|a, b| a.is_subset(b))
    }

    fn has_overlapping_assignments(&self) -> bool {
        self.any_pair_matches(|a, b| a.overlap(b))
    }
}

fn main() {
    let stdin = stdin().lock();
    let assignment_pairs = SectionAssignmentPair::parse_all(stdin);

    println!("{}", assignment_pairs.iter().filter(|p| p.has_redundant_assignment()).count());
    println!("{}", assignment_pairs.iter().filter(|p| p.has_overlapping_assignments()).count());
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use super::*;

    const EXAMPLE: &str = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#;

    #[test]
    fn _01_example() {
        let assignment_pairs = SectionAssignmentPair::parse_all(Cursor::new(EXAMPLE));

        assert_eq!(assignment_pairs.len(), 6);
        assert_eq!(assignment_pairs.iter().filter(|p| p.has_redundant_assignment()).count(), 2);
    }

    #[test]
    fn _02_example() {
        let assignment_pairs = SectionAssignmentPair::parse_all(Cursor::new(EXAMPLE));

        assert_eq!(assignment_pairs.len(), 6);
        assert_eq!(assignment_pairs.iter().filter(|p| p.has_overlapping_assignments()).count(), 4);
    }
}
