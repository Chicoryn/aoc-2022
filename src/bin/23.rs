use std::{io::{BufRead, stdin}, collections::{HashSet, HashMap}, fmt::Debug};

const ELF: char = '#';

#[derive(Clone, Copy)]
enum Direction {
    North,
    South,
    West,
    East
}

impl Direction {
    fn delta(&self) -> (i64, i64) {
        match *self {
            Self::North => (-1, 0),
            Self::South => (1, 0),
            Self::West => (0, -1),
            Self::East => (0, 1),
        }
    }

    fn is_valid(&self) -> impl Iterator<Item=(i64, i64)> {
        match self {
            Self::North => [(-1, -1), (-1, 0), (-1, 1)].into_iter(),
            Self::South => [(1, -1), (1, 0), (1, 1)].into_iter(),
            Self::West => [(-1, -1), (0, -1), (1, -1)].into_iter(),
            Self::East => [(-1, 1), (0, 1), (1, 1)].into_iter(),
        }
    }
}

#[derive(Clone)]
struct Elf {
    x: i64,
    y: i64,
    candidates: [Direction; 4],
}

impl Elf {
    fn new(y: i64, x: i64) -> Self {
        let candidates = [
            Direction::North,
            Direction::South,
            Direction::West,
            Direction::East
        ];

        Self { x, y, candidates }
    }

    fn move_to(&self, y: i64, x: i64) -> Elf {
        Elf { x, y, candidates: self.candidates }
    }

    fn rotate_candidates(&mut self) {
        self.candidates = [
            self.candidates[1],
            self.candidates[2],
            self.candidates[3],
            self.candidates[0],
        ];
    }

    fn adjacents(&self) -> impl Iterator<Item=(i64, i64)> {
        [
            (self.y-1, self.x+1),
            (self.y+0, self.x+1),
            (self.y+1, self.x+1),
            (self.y-1, self.x+0),
            (self.y+1, self.x+0),
            (self.y-1, self.x-1),
            (self.y+0, self.x-1),
            (self.y+1, self.x-1),
        ].into_iter()
    }
}

struct Grove {
    elves: Vec<Elf>,
}

impl Debug for Grove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (min_y, min_x, max_y, max_x) = self.elves.iter()
            .fold((i64::MAX, i64::MAX, i64::MIN, i64::MIN), |(min_y, min_x, max_y, max_x), elf| {
                (
                    min_y.min(elf.y),
                    min_x.min(elf.x),
                    max_y.max(elf.y + 1),
                    max_x.max(elf.x + 1),
                )
            });

        for y in min_y..max_y {
            for x in min_x..max_x {
                if self.elves.iter().any(|elf| elf.y == y && elf.x == x) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }

            writeln!(f, "")?;
        }

        Ok(())
    }
}

impl Grove {
    fn parse(reader: impl BufRead) -> Self {
        let elves = reader.lines()
            .enumerate()
            .flat_map(|(i, line)| {
                line.unwrap()
                    .chars()
                    .enumerate()
                    .filter_map(|(j, ch)| {
                        if ch == ELF {
                            Some(Elf::new(i as i64, j as i64))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
                    .into_iter()
            })
            .collect();

        Self { elves }
    }

    fn rounds(&self, n: usize) -> (Self, usize) {
        let mut elves = self.elves.clone();

        for round_num in 0..n {
            let mut to_move = vec! [];
            let busy = elves.iter()
                .map(|elf| (elf.y, elf.x))
                .collect::<HashSet<_>>();
            let mut occurances = HashMap::new();

            for elf in &elves {
                let (ny, nx) = if elf.adjacents().any(|(y, x)| busy.contains(&(y, x))) {
                    let valid_direction = elf.candidates.iter()
                        .find(|direction| direction.is_valid().all(|(dy, dx)| !busy.contains(&(elf.y+dy, elf.x+dx))));

                    if let Some(direction) = valid_direction {
                        (elf.y + direction.delta().0, elf.x + direction.delta().1)
                    } else {
                        (elf.y, elf.x)
                    }
                } else {
                    (elf.y, elf.x)
                };

                let mut new_elf = elf.clone();
                new_elf.rotate_candidates();

                to_move.push((new_elf, (ny, nx)));
                occurances.entry((ny, nx)).and_modify(|v| *v += 1).or_insert(1);
            }

            let mut moved = false;
            elves = to_move.into_iter()
                .map(|(elf, new_pos)| {
                    if occurances[&new_pos] > 1 {
                        elf
                    } else {
                        moved = moved || elf.y != new_pos.0 || elf.x != new_pos.1;
                        elf.move_to(new_pos.0, new_pos.1)
                    }
                })
                .collect::<Vec<_>>();

            if !moved {
                return (Self { elves }, round_num + 1);
            }
        }

        (Self { elves }, n)
    }

    fn area(&self) -> usize {
        let (min_y, min_x, max_y, max_x) = self.elves.iter()
            .fold((i64::MAX, i64::MAX, i64::MIN, i64::MIN), |(min_y, min_x, max_y, max_x), elf| {
                (
                    min_y.min(elf.y),
                    min_x.min(elf.x),
                    max_y.max(elf.y + 1),
                    max_x.max(elf.x + 1),
                )
            });

        ((max_y - min_y) * (max_x - min_x)) as usize
    }

    fn num_elves(&self) -> usize {
        self.elves.len()
    }

    fn num_empty(&self) -> usize {
        self.area() - self.num_elves()
    }
}

fn main() {
    let stdin = stdin().lock();
    let grove = Grove::parse(stdin);

    println!("{}", grove.rounds(10).0.num_empty());
    println!("{}", grove.rounds(100_000).1);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    const SMALL_EXAMPLE: &str = r#".....
..##.
..#..
.....
..##.
....."#;

    const EXAMPLE: &str = r#"....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#.."#;

    #[test]
    fn _01_small_example() {
        let grove = Grove::parse(Cursor::new(SMALL_EXAMPLE));
        assert_eq!(format!("{:?}", grove.rounds(10).0).trim(), "..#..
....#
#....
....#
.....
..#..");
    }

    #[test]
    fn _01_example() {
        let grove = Grove::parse(Cursor::new(EXAMPLE));

        assert_eq!(format!("{:?}", grove.rounds(10).0).trim(), "......#.....
..........#.
.#.#..#.....
.....#......
..#.....#..#
#......##...
....##......
.#........#.
...#.#..#...
............
...#..#..#..");
        assert_eq!(grove.rounds(10).0.num_empty(), 110);
    }

    #[test]
    fn _02_example() {
        let (grove, n) = Grove::parse(Cursor::new(EXAMPLE)).rounds(1000);

        assert_eq!(format!("{:?}", grove).trim(), ".......#......
....#......#..
..#.....#.....
......#.......
...#....#.#..#
#.............
....#.....#...
..#.....#.....
....#.#....#..
.........#....
....#......#..
.......#......");
        assert_eq!(n, 20);
    }
}
