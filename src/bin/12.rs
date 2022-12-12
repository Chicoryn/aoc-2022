use ndarray::prelude::*;
use std::io::{prelude::*, stdin};
use std::collections::VecDeque;

struct HMap {
    raw_values: Array2<char>,
    heights: Array2<usize>,
}

impl HMap {
    fn parse<R: BufRead>(reader: R) -> Self {
        let lines = reader.lines().filter_map(|line| line.ok()).map(|line| {
            Array1::from_shape_vec((line.len(),), line.chars().map(|ch| ch).collect()).unwrap()
        }).collect::<Vec<_>>();
        let raw_values = ndarray::stack(
            Axis(0),
            &lines.iter().map(|line| line.view()).collect::<Vec<_>>()
        ).unwrap();
        let heights = raw_values.map(|ch| {
            (match ch {
                'S' => 'a',
                'E' => 'z',
                ch => *ch
            }) as usize - 'a' as usize
        });

        Self {
            raw_values,
            heights,
        }
    }

    fn starting_point(&self) -> (usize, usize) {
        self.raw_values.indexed_iter().filter(|(_, &value)| value == 'S').map(|(point, _)| point).next().unwrap()
    }

    fn possible_starting_points<'a>(&'a self) -> impl Iterator<Item=(usize, usize)> + 'a {
        self.heights.indexed_iter().filter(|(_, &value)| value == 0).map(|(point, _)| point)
    }

    fn goal_point(&self) -> (usize, usize) {
        self.raw_values.indexed_iter().filter(|(_, &value)| value == 'E').map(|(point, _)| point).next().unwrap()
    }

    fn neighbours(&self, point: (usize, usize)) -> impl Iterator<Item=(usize, usize)> {
        const DELTA: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        let (shape_i, shape_j) = self.heights.dim();

        DELTA.iter()
            .map(move |(di, dj)| (point.0 as isize + *di, point.1 as isize + *dj))
            .filter(move |(i, j)| {
                *i >= 0 && *j >= 0 && *i < shape_i as isize && *j < shape_j as isize
            })
            .map(|(i, j)| { (i as usize, j as usize) })
    }

    fn shortest_paths(&self, starting_point: (usize, usize)) -> Array2<usize> {
        let shape  = self.heights.dim();
        let mut shortest_so_far = Array2::from_elem(shape, usize::MAX);
        let mut to_visit = VecDeque::new();
        to_visit.push_back(starting_point);
        shortest_so_far[starting_point] = 0;

        while let Some(point) = to_visit.pop_front() {
            let curr_distance = shortest_so_far[point];
            let curr_height = self.heights[point];

            for neighbour in self.neighbours(point) {
                if self.heights[neighbour] <= curr_height + 1 && shortest_so_far[neighbour] > curr_distance + 1 {
                    shortest_so_far[neighbour] = curr_distance + 1;
                    to_visit.push_back(neighbour);
                }
            }
        }

        shortest_so_far
    }
}

fn main() {
    let stdin = stdin().lock();
    let hmap = HMap::parse(stdin);
    let goal_point = hmap.goal_point();

    println!("{}", hmap.shortest_paths(hmap.starting_point())[goal_point]);
    println!("{}", hmap.possible_starting_points().map(|starting_point| hmap.shortest_paths(starting_point)[goal_point]).min().unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    const EXAMPLE: &str = r#"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"#;

    #[test]
    fn _01_example() {
        let hmap = HMap::parse(Cursor::new(EXAMPLE));
        let min_distance_to = hmap.shortest_paths(hmap.starting_point());

        assert_eq!(min_distance_to[hmap.goal_point()], 31);
    }

    #[test]
    fn _02_example() {
        let hmap = HMap::parse(Cursor::new(EXAMPLE));
        let goal_point = hmap.goal_point();

        assert_eq!(hmap.possible_starting_points().map(|starting_point| hmap.shortest_paths(starting_point)[goal_point]).min(), Some(29));
    }
}