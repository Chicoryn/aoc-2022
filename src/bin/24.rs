use std::{io::{prelude::*, stdin}, collections::{HashSet, VecDeque}, fmt::Display};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl TryFrom<char> for Direction {
    type Error = ();

    fn try_from(ch: char) -> Result<Self, Self::Error> {
        match ch {
            '>' => Ok(Direction::East),
            '<' => Ok(Direction::West),
            '^' => Ok(Direction::North),
            'v' => Ok(Direction::South),
            _ => Err(()),
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::North => "^",
            Self::West => "<",
            Self::South => "v",
            Self::East => ">",
        })
    }
}

impl Direction {
    fn all() -> impl Iterator<Item=Direction> {
        [
            Self::North,
            Self::East,
            Self::South,
            Self::West
        ].into_iter()
    }

    fn delta(self) -> (i64, i64) {
        match self {
            Self::North => (-1, 0),
            Self::East => (0, 1),
            Self::South => (1, 0),
            Self::West => (0, -1),
        }
    }
}

struct Blizzard {
    direction: Direction,
    starts_at: (i64, i64),
    dims: (i64, i64),
}

impl Blizzard {
    fn new(y: usize, x: usize, dims: (usize, usize), direction: Direction) -> Self {
        Self {
            direction,
            starts_at: (y as i64, x as i64),
            dims: (dims.0 as i64, dims.1 as i64),
        }
    }

    #[cfg(test)]
    fn direction(&self) -> Direction {
        self.direction
    }

    fn position_at(&self, time: usize) -> (usize, usize) {
        let (sy, sx) = self.starts_at;
        let (dy, dx) = self.direction.delta();
        let time = time as i64;
        let (y, x) = (
            ((sy - 1) + time * dy) % (self.dims.0 - 2),
            ((sx - 1) + time * dx) % (self.dims.1 - 2),
        );

        if y < 0 {
            ((self.dims.0 - 1 + y) as usize, 1 + x as usize)
        } else if x < 0 {
            (1 + y as usize, (self.dims.1 - 1 + x) as usize)
        } else {
            (1 + y as usize, 1 + x as usize)
        }
    }
}

struct Valley {
    rows: Vec<Vec<Blizzard>>,
    cols: Vec<Vec<Blizzard>>,
    walls: HashSet<(usize, usize)>,
    dims: (usize, usize),
}

impl Valley {
    fn parse(reader: impl BufRead) -> Self {
        let lines = reader.lines()
            .filter_map(|line| line.ok())
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate()
                    .map(|(x, ch)| ((y, x), ch))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let (height, width) = lines.iter()
            .fold((0, 0), |(height, width), ((y, x), _)| (height.max(y + 1), width.max(x + 1)));

        Self {
            rows: (0..height)
                .map(|row| Self::collect_blizzards(&lines, (height, width), |y, _| y == row))
                .collect(),
            cols: (0..width)
                .map(|col| Self::collect_blizzards(&lines, (height, width), |_, x| x == col))
                .collect(),
            walls: lines.iter()
                .filter(|(_, ch)| *ch == '#')
                .map(|(pos, _)| *pos)
                .collect(),
            dims: (height, width),
        }
    }

    fn collect_blizzards(lines: &[((usize, usize), char)], dims: (usize, usize), check: impl Fn(usize, usize) -> bool) -> Vec<Blizzard> {
        lines.iter()
            .filter(|((y, x), _)| check(*y, *x))
            .filter_map(|((y, x), ch)| Direction::try_from(*ch).ok().map(|dir| ((y, x), dir)))
            .map(|((y, x), dir)| Blizzard::new(*y, *x, dims, dir))
            .collect::<Vec<_>>()
    }

    fn start_point(&self) -> (usize, usize) {
        (
            0,
            (0..self.dims.1)
                .filter(|x| !self.walls.contains(&(0, *x)))
                .next()
                .unwrap()
        )
    }

    fn end_point(&self) -> (usize, usize) {
        let y = self.dims.0 - 1;

        (
            y,
            (0..self.dims.1)
                .filter(|x| !self.walls.contains(&(y, *x)))
                .next()
                .unwrap()
        )
    }

    #[cfg(test)]
    fn display_at(&self, time: usize) -> String {
        let mut f = String::new();

        for row in 0..self.dims.0 {
            for col in 0..self.dims.1 {
                let blizzards = self.rows[row].iter()
                    .filter(|blizzard| blizzard.position_at(time) == (row, col))
                    .chain(
                        self.cols[col].iter()
                            .filter(|blizzard| blizzard.position_at(time) == (row, col))
                    )
                    .collect::<Vec<_>>();
                let mut directions = blizzards.iter()
                    .map(|blizzard| blizzard.direction())
                    .collect::<Vec<_>>();
                directions.sort_unstable();
                directions.dedup();

                if directions.is_empty() {
                    f += if self.walls.contains(&(row, col)) { "#" } else { "." };
                } else if directions.len() == 1 {
                    f += &format!("{}", directions[0]);
                } else {
                    f += &format!("{}", directions.len());
                }
            }

            f += "\n";
        }

        f.trim().to_string()
    }

    fn is_empty_at(&self, position: (usize, usize), time: usize) -> bool {
        position.0 < self.dims.0 && position.1 < self.dims.1 &&
            (!self.walls.contains(&position) &&
            self.rows[position.0].iter()
                .chain(self.cols[position.1].iter())
                .all(|blizzard| blizzard.position_at(time) != position))
    }
}

fn manhattan_distance(start_at: (usize, usize), end_at: (usize, usize)) -> usize {
    (start_at.0.max(end_at.0) - start_at.0.min(end_at.0))
        + (start_at.1.max(end_at.1) - start_at.1.min(end_at.1))
}

fn shortest_path(valley: &Valley, start_at: (usize, usize), start_time: usize, end_at: (usize, usize)) -> usize {
    let mut so_far = usize::MAX;
    let mut visited = HashSet::new();
    let mut to_visit = VecDeque::new();
    to_visit.push_back((start_at, start_time));

    while let Some((position, t)) = to_visit.pop_front() {
        if position == end_at {
            so_far = so_far.min(t);
            continue; // best so far?
        } else if manhattan_distance(position, end_at) + t > so_far {
            continue; // worse than best so far
        } else if !valley.is_empty_at(position, t) {
            continue; // hit by blizzard
        }

        for next_direction in Direction::all() {
            let (dy, dx) = next_direction.delta();
            let next_position = (
                (position.0 as i64 + dy) as usize,
                (position.1 as i64 + dx) as usize,
            );

            if valley.is_empty_at(next_position, t + 1) && visited.insert((next_position, t + 1)) {
                to_visit.push_back((next_position, t + 1));
            }
        }

        if valley.is_empty_at(position, t + 1) && visited.insert((position, t + 1)) {
            to_visit.push_back((position, t + 1));
        }
    }

    so_far
}

fn shortest_path3(valley: &Valley) -> usize {
    let start_at = valley.start_point();
    let end_at = valley.end_point();

    let t = shortest_path(valley, start_at, 0, end_at);
    let t = shortest_path(valley, end_at, t, start_at);
    shortest_path(valley, start_at, t, end_at)
}

fn main() {
    let stdin = stdin().lock();
    let valley = Valley::parse(stdin);

    eprintln!("{}", shortest_path(&valley, valley.start_point(), 0, valley.end_point()));
    eprintln!("{}", shortest_path3(&valley));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    const SMALL_EXAMPLE: &str = r#"#.#####
#.....#
#>....#
#.....#
#...v.#
#.....#
#####.#"#;

    const EXAMPLE: &str = r#"#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#"#;

    #[test]
    fn _01_small_example() {
        let valley = Valley::parse(Cursor::new(SMALL_EXAMPLE));

        assert_eq!(valley.display_at(2), "#.#####
#...v.#
#..>..#
#.....#
#.....#
#.....#
#####.#");
        assert_eq!(valley.display_at(5), "#.#####
#.....#
#>....#
#.....#
#...v.#
#.....#
#####.#");
    }

    #[test]
    fn _01_example() {
        let valley = Valley::parse(Cursor::new(EXAMPLE));

        assert_eq!(valley.start_point(), (0, 1));
        assert_eq!(valley.end_point(), (5, 6));
        assert_eq!(shortest_path(&valley, valley.start_point(), 0, valley.end_point()), 18);
    }

    #[test]
    fn _02_example() {
        let valley = Valley::parse(Cursor::new(EXAMPLE));
        assert_eq!(shortest_path3(&valley), 54);
    }
}