use sscanf::sscanf;
use std::io::{prelude::*, stdin};
use ndarray::prelude::*;

#[derive(Clone)]
struct Path {
    points: Vec<(usize, usize)>
}

impl Path {
    fn parse_all<R: BufRead>(reader: R) -> Vec<Self> {
        reader.lines()
            .filter_map(|line| line.ok())
            .map(|line| Path::parse(&line))
            .collect::<Vec<_>>()
    }

    fn parse(line: &str) -> Self {
        let points = line.split("->")
            .filter_map(|part| sscanf!(part.trim(), "{},{}", usize, usize).ok())
            .collect::<Vec<_>>();

        Self { points }
    }

    fn new(from: (usize, usize), to: (usize, usize)) -> Self {
        Self {
            points: vec! [from, to]
        }
    }

    fn fill_matrix(&self, fill_to: &mut Array2<bool>) {
        for i in 0..(self.points.len() - 1) {
            let x0 = self.points[i];
            let x1 = self.points[i+1];

            for i in x0.0.min(x1.0)..=x0.0.max(x1.0) {
                for j in x0.1.min(x1.1)..=x0.1.max(x1.1) {
                    fill_to[(i, j)] = true;
                }
            }
        }
    }

    fn bounding_box(&self) -> (usize, usize) {
        self.points.iter()
            .fold((0, 0), |acc, point| {
                (acc.0.max(point.0), acc.1.max(point.1))
            })
    }
}

#[derive(Clone, Copy)]
struct Sand(usize, usize);

impl Sand {
    fn try_fall(self, check: impl Fn(usize, usize) -> bool) -> Option<Sand> {
        let Sand(i, j) = self;

        if !check(i, j + 1) {
            Some(Sand(i, j + 1))
        } else if !check(i - 1, j + 1) {
            Some(Sand(i - 1, j + 1))
        } else if !check(i + 1, j + 1) {
            Some(Sand(i + 1, j + 1))
        } else {
            None
        }
    }
}

struct Cave {
    structure: Array2<bool>
}

impl Cave {
    fn from_paths(paths: Vec<Path>) -> Self {
        let bounding_box = paths.iter()
            .fold((0, 0), |acc, path| {
                let bb = path.bounding_box();
                (acc.0.max(bb.0), acc.1.max(bb.1))
            });
        let mut structure = Array2::from_elem((bounding_box.0 + 1, bounding_box.1 + 1), false);

        for path in &paths {
            path.fill_matrix(&mut structure);
        }

        Self { structure }
    }

    fn bounding_box(&self) -> (usize, usize) {
        self.structure.dim()
    }

    fn intersects(&self, point: (usize, usize)) -> bool {
        point.0 < self.structure.dim().0
            && point.1 < self.structure.dim().1
            && self.structure[point]
    }

    fn drop_at(&mut self, mut starting_point: Sand) -> bool {
        let bounding_box = self.structure.dim();

        while starting_point.0 < bounding_box.0 && starting_point.1 < bounding_box.1 {
            if let Some(new_point) = starting_point.try_fall(|i, j| self.intersects((i, j))) {
                starting_point = new_point;
            } else {
                let point = (starting_point.0, starting_point.1);

                if self.structure[point] != true {
                    self.structure[point] = true;
                    return true
                } else {
                    return false
                }
            }
        }

        false
    }

    fn drop_until_full(&mut self, starting_point: Sand) -> usize {
        let mut count = 0;

        while self.drop_at(starting_point) {
            count += 1;
        }

        count
    }
}

fn main() {
    let stdin = stdin().lock();
    let paths = Path::parse_all(stdin);
    let mut cave = Cave::from_paths(paths.clone());
    let mut cave_with_floor = Cave::from_paths([
        paths,
        vec! [
            Path::new(
                (0, cave.bounding_box().1 + 1),
                (cave.bounding_box().0 + cave.bounding_box().1 + 1, cave.bounding_box().1 + 1),
            )
        ]
    ].concat());

    println!("{}", cave.drop_until_full(Sand(500, 0)));
    println!("{}", cave_with_floor.drop_until_full(Sand(500, 0)));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    const EXAMPLE: &str = r#"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"#;

    #[test]
    fn _01_example() {
        let mut cave = Cave::from_paths(Path::parse_all(Cursor::new(EXAMPLE)));
        assert_eq!(cave.drop_until_full(Sand(500, 0)), 24);
    }

    #[test]
    fn _02_example() {
        let paths = Path::parse_all(Cursor::new(EXAMPLE));
        let cave = Cave::from_paths(paths.clone());
        let mut cave_with_floor = Cave::from_paths([
            paths,
            vec! [
                Path::new(
                    (0, cave.bounding_box().1 + 1),
                    (cave.bounding_box().0 + cave.bounding_box().1 + 1, cave.bounding_box().1 + 1),
                )
            ]
        ].concat());

        assert_eq!(cave_with_floor.drop_until_full(Sand(500, 0)), 93);
    }
}
