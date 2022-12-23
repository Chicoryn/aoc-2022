use std::{io::{prelude::*, stdin}, iter};
use ndarray::{prelude::*, stack};

const NAN: char = ' ';
const WALL: char = '#';

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    fn password(&self) -> i64 {
        match self {
            Self::Up => 1,
            Self::Right => 0,
            Self::Down => 3,
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

    fn opposite(&self) -> Direction {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left
        }
    }

    fn index(&self) -> usize {
        match self {
            Self::Right => 0,
            Self::Down => 1,
            Self::Left => 2,
            Self::Up => 3,
        }
    }

    fn from_index(index: usize) -> Direction {
        match index {
            0 => Self::Right,
            1 => Self::Down,
            2 => Self::Left,
            3 => Self::Up,
            _ => panic!()
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
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
    fn fix(&self, position: Position) -> Position;
    fn oob(&self, prev_position: Position, position: Position) -> Position;
}

/// A 3D cube is always has exactly six sides, each of which are always
/// connected to four other sides. Given the following configuration the
/// following sides are connected (and their flip-side):
///
/// * A -> B (Left)
/// * A -> C (Up)
/// * A -> D (Right)
/// * A -> F (Down)
/// * B -> C (Right)
/// * B -> E (Up)
/// * B -> F (Left)
/// * C -> D (Right)
/// * C -> E (Up)
/// * D -> E (Up)
/// * D -> F (Right)
/// * E -> F (Up)
///
/// ```
///    A
///  B C D
///    E
///    F
/// ```
///
/// We need to assign each side to a letter by folding the squares based on
/// given constraints. We can do this by exhaustive search, since there are only
/// `6! = 720` assigments.
struct FoldedBoundsCheck {
    map: Array2<char>,
    squares: Array2<i8>,
    square_size: usize,
    connected_sides: [[usize; 4]; 6],
}

impl FoldedBoundsCheck {
    fn new(map: &Array2<char>, connected_sides: [[usize; 4]; 6]) -> Self {
        let n = Self::largest_cube(map).unwrap();
        let squares = Self::split_into_squares(map, n);
        debug_assert!(map.dim().0 % n == 0);
        debug_assert!(map.dim().1 % n == 0);

        Self {
            map: map.clone(),
            squares,
            square_size: n,
            connected_sides
        }
    }

    fn largest_cube(map: &Array2<char>) -> Option<usize> {
        let (mut y, x) = (0, 0);

        while map[(y, x)] == NAN {
            y += 1;
        }

        let n = (map.dim().0 - y).min(map.dim().1);

        (1..n).rev()
            .filter(|n| {
                map.slice(s! [
                    y..(y + n).min(map.dim().0),
                    x..(x + n).min(map.dim().1)
                ]).iter().all(|&v| v != NAN)
            })
            .next()
    }

    fn split_into_squares(map: &Array2<char>, n: usize) -> Array2<i8> {
        let mut squares = Array2::from_elem(map.dim(), -1);
        let mut count = 0;

        for y in 0..map.dim().0 {
            for x in 0..map.dim().1 {
                let map_slice = map.slice(s! [
                    y..(y + n).min(map.dim().0),
                    x..(x + n).min(map.dim().1)
                ]);

                if map_slice.indexed_iter().all(|((sy, sx), &v)| squares[(y+sy, x+sx)] < 0 && v != NAN) {
                    let mut square_slice = squares.slice_mut(s! [
                        y..(y + n).min(squares.dim().0),
                        x..(x + n).min(squares.dim().1)
                    ]);

                    for s in square_slice.iter_mut() {
                        *s = count;
                    }

                    count += 1;
                }
            }
        }
        debug_assert_eq!(count, 6);

        squares
    }

    fn relative_to_abs_in(&self, in_square: usize, rel_y: usize, rel_x: usize, dir: Direction) -> Position {
        let (y, x) = (0..self.squares.dim().0)
            .find_map(|i| {
                (0..self.squares.dim().1)
                    .find(|&j| self.squares[(i, j)] == in_square as i8)
                    .map(|j| (i, j))
            })
            .unwrap();

        Position((y + rel_y) as i64, (x + rel_x) as i64, dir)
    }

    fn move_from_aux(&self, from_square: usize, from_side: Direction, to_square: usize, to_side: Direction, from_y: i64, from_x: i64) -> Position {
        let rel_x = self.squares.slice(s! [
            from_y as usize,
            0..(from_x as usize),
        ]).iter().filter(|&&x| x == from_square as i8).count();
        let rel_y = self.squares.slice(s! [
            0..(from_y as usize),
            from_x as usize,
        ]).iter().filter(|&&x| x == from_square as i8).count();
        let n = self.square_size - 1;

        match (from_side, to_side) {
            (Direction::Down, Direction::Down) => self.relative_to_abs_in(to_square, 0, rel_x, to_side.opposite()),
            (Direction::Down, Direction::Up) => self.relative_to_abs_in(to_square, n, rel_x, to_side.opposite()),
            (Direction::Down, Direction::Left) => self.relative_to_abs_in(to_square, rel_x, 0, to_side.opposite()),
            (Direction::Down, Direction::Right) => self.relative_to_abs_in(to_square, n - rel_x, n, to_side.opposite()),

            (Direction::Up, Direction::Down) => self.relative_to_abs_in(to_square, 0, rel_x, to_side.opposite()),
            (Direction::Up, Direction::Up) => self.relative_to_abs_in(to_square, n, n - rel_x, to_side.opposite()),
            (Direction::Up, Direction::Left) => self.relative_to_abs_in(to_square, n - rel_x, 0, to_side.opposite()),
            (Direction::Up, Direction::Right) => self.relative_to_abs_in(to_square, rel_x, n, to_side.opposite()),

            (Direction::Left, Direction::Down) => self.relative_to_abs_in(to_square, 0, rel_y, to_side.opposite()),
            (Direction::Left, Direction::Up) => self.relative_to_abs_in(to_square, n, n - rel_y, to_side.opposite()),
            (Direction::Left, Direction::Left) => self.relative_to_abs_in(to_square, n - rel_y, 0, to_side.opposite()),
            (Direction::Left, Direction::Right) => self.relative_to_abs_in(to_square, rel_y, n, to_side.opposite()),

            (Direction::Right, Direction::Down) => self.relative_to_abs_in(to_square, 0, n - rel_y, to_side.opposite()),
            (Direction::Right, Direction::Up) => self.relative_to_abs_in(to_square, n, rel_y, to_side.opposite()),
            (Direction::Right, Direction::Left) => self.relative_to_abs_in(to_square, rel_y, 0, to_side.opposite()),
            (Direction::Right, Direction::Right) => self.relative_to_abs_in(to_square, n - rel_y, n, to_side.opposite()),
        }
    }

    fn move_from(&self, from_square: usize, from_y: i64, from_x: i64, from_side: Direction) -> Position {
        let to_square = self.connected_sides[from_square][from_side.index()];
        let to_side = Direction::from_index(self.connected_sides[to_square].iter().position(|&v| v == from_square).unwrap());

        self.move_from_aux(from_square, from_side, to_square, to_side, from_y, from_x)
    }
}

impl BoundsCheck for FoldedBoundsCheck {
    fn fix(&self, mut pos: Position) -> Position {
        while self.map[(pos.0 as usize, pos.1 as usize)] == NAN {
            let (dy, dx) = pos.2.delta();
            pos = Position(pos.0 + dy, pos.1 + dx, pos.2);
        }

        pos
    }

    fn oob(&self, prev_pos: Position, pos: Position) -> Position {
        let Position(y, x, dir) = pos;
        let (h, w) = self.squares.dim();

        if y < 0 || y >= h as i64 || x < 0 || x >= w as i64 {
            self.move_from(self.squares[(prev_pos.0 as usize, prev_pos.1 as usize)] as usize, prev_pos.0, prev_pos.1, dir)
        } else if self.squares[(y as usize, x as usize)] < 0 {
            self.move_from(self.squares[(prev_pos.0 as usize, prev_pos.1 as usize)] as usize, prev_pos.0, prev_pos.1, dir)
        } else {
            Position(y, x, dir)
        }
    }
}

struct SimpleBoundsCheck {
    map: Array2<char>,
    width: i64,
    height: i64,
}

impl SimpleBoundsCheck {
    fn new(map: Array2<char>, height: i64, width: i64) -> Self {
        Self { map, height, width }
    }

    fn simple_oob(&self, pos: Position) -> Position {
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

impl BoundsCheck for SimpleBoundsCheck {
    fn fix(&self, mut pos: Position) -> Position {
        while self.map[(pos.0 as usize, pos.1 as usize)] == NAN {
            let (dy, dx) = pos.2.delta();
            pos = self.simple_oob(Position(pos.0 + dy, pos.1 + dx, pos.2));
        }

        pos
    }

    fn oob(&self, _: Position, pos: Position) -> Position {
        self.fix(self.simple_oob(pos))
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
        let map = stack(
            Axis(0),
            &lines.iter().map(|line| line.view()).collect::<Vec<_>>()
        ).unwrap();

        Self {
            bounds_check: Box::new(SimpleBoundsCheck::new(map.clone(), lines.len() as i64, max_len as i64)),
            map
        }
    }

    fn fold(&self, connected_sides: [[usize; 4]; 6]) -> Self {
        Self {
            bounds_check: Box::new(FoldedBoundsCheck::new(&self.map, connected_sides)),
            map: self.map.clone(),
        }
    }

    fn take_step(&self, pos: Position, command: Command) -> Position {
        let mut pos = self.bounds_check.fix(pos);

        match command {
            Command::Move(n) => {
                for _ in 0..n {
                    let (dy, dx) = pos.2.delta();
                    let (ny, nx) = (pos.0 + dy, pos.1 + dx);
                    let new_pos = self.bounds_check.oob(pos, Position(ny, nx, pos.2));

                    if self.map[(new_pos.0 as usize, new_pos.1 as usize)] != WALL {
                        pos = new_pos;
                    }
                }

                pos
            },
            Command::Left => Position(pos.0, pos.1, pos.2.turn_left()),
            Command::Right => Position(pos.0, pos.1, pos.2.turn_right()),
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
    let folded_map = map.fold([
        // R, D, L, U
        [  1, 5, 3, 2],
        [  4, 5, 0, 2],
        [  1, 0, 3, 4],
        [  4, 2, 0, 5],
        [  1, 2, 3, 5],
        [  4, 3, 0, 1],
    ]);
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
        let map = Map::parse(&mut example).fold([
            // R, D, L, U
            [  5, 1, 2, 3],
            [  2, 0, 5, 4],
            [  3, 0, 1, 4],
            [  5, 0, 2, 4],
            [  5, 3, 2, 1],
            [  0, 3, 4, 1],
        ]);
        let path = Path::parse(&mut example);

        assert_eq!(path.iter().fold(Position::starting_position(), |prev, cmd| map.take_step(prev, cmd)).password(), 5031);
    }
}