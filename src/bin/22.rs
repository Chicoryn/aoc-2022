use std::{io::{prelude::*, stdin}, iter};
use ndarray::{prelude::*, stack};

const NAN: char = ' ';
const EMPTY: char = '.';
const WALL: char = '#';

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    fn password(&self) -> i64 {
        match self {
            Self::Up => 3,
            Self::Right => 0,
            Self::Down => 1,
            Self::Left => 2,
        }
    }

    fn delta(self) -> (i64, i64) {
        match self {
            Self::Up => (1, 0),
            Self::Down => (-1, 0),
            Self::Left => (0, -1),
            Self::Right => (0, 1),
        }
    }

    fn turn_left(&self) -> Direction {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }

    fn turn_right(&self) -> Direction {
        match self {
            Self::Up => Self::Left,
            Self::Left => Self::Down,
            Self::Down => Self::Right,
            Self::Right => Self::Up,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Position(i64, i64, Direction);

impl Position {
    fn starting_position() -> Self {
        Self(0, 0, Direction::Right)
    }

    fn password(&self) -> i64 {
        1000 * (1 + self.0) + 4 * (1 + self.1) + self.2.password()
    }
}

trait BoundsCheck {
    fn oob(&self, position: Position) -> Position;
}

/// A 3D cube is always has exactly six sides, each of which are always
/// connected to four other sides. Given the following configuration the
/// following sides are connected:
///
/// * A <-> B
/// * A <-> C
/// * A <-> D
/// * A <-> F
/// * B <-> D
/// * B <-> 
///
/// ```
///    A
///  B C D
///    E
///    F
/// ```
struct FoldedBoundsCheck {
    // pass
}

impl FoldedBoundsCheck {
    fn new(map: &Array2<char>) -> Self {
        // split into six squares
        // figure out how the six squares are connected
        //
        Self {}
    }
}

impl BoundsCheck for FoldedBoundsCheck {
    fn oob(&self, pos: Position) -> Position {
        pos
    }
}

struct SimpleBoundsCheck {
    width: i64,
    height: i64,
}

impl SimpleBoundsCheck {
    fn new(height: i64, width: i64) -> Self {
        Self { height, width }
    }
}

impl BoundsCheck for SimpleBoundsCheck {
    fn oob(&self, pos: Position) -> Position {
        let (h, w) = (self.height, self.width);

        if pos.0 < 0 {
            Position(h + pos.0, pos.1, pos.2)
        } else if pos.0 >= h {
            Position(pos.0 - h, pos.1, pos.2)
        } else if pos.1 < 0 {
            Position(pos.0, w + pos.1, pos.2)
        } else if pos.1 >= w {
            Position(pos.0, pos.1 - w, pos.2)
        } else {
            pos
        }
    }
}

struct Map {
    bounds_check: Box<dyn BoundsCheck>,
    map: Array2<char>,
}

impl Map {
    fn parse(reader: &mut impl BufRead) -> Self {
        let lines = reader.lines()
            .filter_map(|line| line.ok())
            .take_while(|line| !line.is_empty())
            .map(|line| Array1::from_vec(line.chars().collect::<Vec<_>>()))
            .collect::<Vec<_>>();
        let max_len = lines.iter()
            .map(|line| line.len())
            .max()
            .unwrap();
        let lines = lines.into_iter()
            .map(|mut line| {
                if max_len > line.dim() {
                    line.append(Axis(0), Array1::from_vec(vec! [NAN; max_len - line.dim()]).view()).unwrap();
                    line
                } else {
                    line
                }
            })
            .collect::<Vec<_>>();

        Self {
            bounds_check: Box::new(SimpleBoundsCheck::new(lines.len() as i64, max_len as i64)),
            map: stack(
                Axis(0),
                &lines.iter().map(|line| line.view()).collect::<Vec<_>>()
            ).unwrap()
        }
    }

    fn fold(&self) -> Self {
        Self {
            bounds_check: Box::new(FoldedBoundsCheck::new(&self.map)),
            map: self.map.clone(),
        }
    }

    fn fix(&self, mut pos: Position) -> Position {
        pos = self.bounds_check.oob(pos);

        while self.map[(pos.0 as usize, pos.1 as usize)] == NAN {
            let (dy, dx) = pos.2.delta();
            pos = self.bounds_check.oob(Position(pos.0 + dy, pos.1 + dx, pos.2));
        }

        pos
    }

    fn take_step(&self, pos: Position, command: Command) -> Position {
        let mut pos = self.fix(pos);
        let direction = match command {
            Command::Left => pos.2.turn_left(),
            Command::Right => pos.2.turn_right(),
            _ => pos.2
        };

        match command {
            Command::Move(n) => {
                let (dy, dx) = direction.delta();

                for _ in 0..n {
                    let (ny, nx) = (pos.0 + dy, pos.1 + dx);
                    let new_pos = self.fix(Position(ny, nx, direction));

                    if self.map[(new_pos.0 as usize, new_pos.1 as usize)] != WALL {
                        //eprintln!("{:?} -> {:?}", pos, new_pos);
                        pos = new_pos;
                    } else {
                        //eprintln!("{:?} -> {:?} (fail)", pos, new_pos);
                    }
                }

                pos
            },
            _ => Position(pos.0, pos.1, direction)
        }

    }
}

#[derive(Debug, PartialEq, Eq)]
enum Command {
    Left,
    Right,
    Move(i64)
}

struct Path {
    text: Vec<char>
}

impl Path {
    fn parse(reader: &mut impl BufRead) -> Self {
        let text = reader.lines().next().unwrap().unwrap().chars().collect();

        Self { text }
    }

    fn iter<'a>(&'a self) -> impl Iterator<Item=Command> + 'a {
        let text = &self.text;
        let mut pos = 0;

        iter::from_fn(move || {
            if pos >= self.text.len() {
                None
            } else {
                match text[pos] {
                    'L' => { pos += 1; Some(Command::Left) },
                    'R' => { pos += 1; Some(Command::Right) },
                    x if x.is_digit(10) => {
                        let start = pos;
                        let end = (pos..text.len())
                            .position(|i| !text[i].is_digit(10))
                            .map(|n| n + start)
                            .unwrap_or(text.len());

                        pos = end;
                        Some(Command::Move(text[start..end].iter().collect::<String>().parse::<i64>().unwrap()))
                    },
                    _ => panic!()
                }
            }
        })
    }
}

fn main() {
    let mut stdin = stdin().lock();
    let map = Map::parse(&mut stdin);
    let folded_map = map.fold();
    let path = Path::parse(&mut stdin);

    println!("{}", path.iter().fold(Position::starting_position(), |prev, cmd| map.take_step(prev, cmd)).password());
    println!("{}", path.iter().fold(Position::starting_position(), |prev, cmd| folded_map.take_step(prev, cmd)).password());
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    const EXAMPLE: &str = r#"        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5"#;

    #[test]
    fn _01_example() {
        let mut example = Cursor::new(EXAMPLE);
        let map = Map::parse(&mut example);
        let path = Path::parse(&mut example);

        assert_eq!(path.iter().fold(Position::starting_position(), |prev, cmd| map.take_step(prev, cmd)).password(), 6032);
    }

    #[test]
    fn _02_example() {
        let mut example = Cursor::new(EXAMPLE);
        let map = Map::parse(&mut example).fold();
        let path = Path::parse(&mut example);

        assert_eq!(path.iter().fold(Position::starting_position(), |prev, cmd| map.take_step(prev, cmd)).password(), 5031);
    }
}