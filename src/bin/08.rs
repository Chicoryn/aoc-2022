use std::io::{prelude::*, stdin};
use ndarray::*;

struct Forest {
    /// column-major order
    trees: Array2<u32>
}

impl Forest {
    fn parse_all<R: BufRead>(reader: R) -> Self {
        let rows = reader.lines()
            .filter_map(|line| line.ok())
            .map(|line| Array::from_iter(line.chars().filter_map(|ch| ch.to_digit(10))))
            .collect::<Vec<_>>();
        let views = rows.iter()
            .map(|row| row.view())
            .collect::<Vec<_>>();

        Self {
            trees: stack(Axis(1), &views).unwrap()
        }
    }

    fn all<'a>(&'a self) -> impl Iterator<Item = Tree<'a>> + 'a {
        self.trees.indexed_iter()
            .map(|(index, &height)| Tree::new(&self.trees, index, height))
    }
}

struct Tree<'a> {
    forest: &'a Array2<u32>,
    index: (usize, usize),
    height: u32
}

impl<'a> Tree<'a> {
    fn new(forest: &'a Array2<u32>, index: (usize, usize), height: u32) -> Self {
        Self { forest, index, height }
    }

    fn is_visible(&self) -> bool {
        let (i, j) = self.index;

        self.forest.slice(s![..i, j]).iter().all(|&other| other < self.height)
            || self.forest.slice(s![(i+1).., j]).iter().all(|&other| other < self.height)
            || self.forest.slice(s![i, ..j]).iter().all(|&other| other < self.height)
            || self.forest.slice(s![i, (j+1)..]).iter().all(|&other| other < self.height)
    }

    fn view_distance(&self, iter: impl Iterator<Item = &'a u32>) -> usize {
        let mut distance = 0;

        for &other_height in iter {
            distance += 1;

            if other_height >= self.height {
                break
            }
        }

        distance
    }

    fn scenic_score(&self) -> usize {
        let (i, j) = self.index;

        self.view_distance(self.forest.slice(s![..i, j]).iter().rev())
            * self.view_distance(self.forest.slice(s![(i+1).., j]).iter())
            * self.view_distance(self.forest.slice(s![i, ..j]).iter().rev())
            * self.view_distance(self.forest.slice(s![i, (j+1)..]).iter())
    }
}

fn main() {
    let stdin = stdin().lock();
    let forest = Forest::parse_all(stdin);

    println!("{}", forest.all().filter(|tree| tree.is_visible()).count());
    println!("{}", forest.all().map(|tree| tree.scenic_score()).max().unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    const EXAMPLE: &str = r#"30373
25512
65332
33549
35390"#;

    #[test]
    fn _01_example() {
        let forest = Forest::parse_all(Cursor::new(EXAMPLE));

        assert_eq!(forest.all().filter(|tree| tree.is_visible()).count(), 21);
    }

    #[test]
    fn _02_example() {
        let forest = Forest::parse_all(Cursor::new(EXAMPLE));

        assert_eq!(forest.all().map(|tree| tree.scenic_score()).max(), Some(8));
    }
}
