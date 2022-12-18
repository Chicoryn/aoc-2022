use sscanf::sscanf;
use std::{collections::{HashSet, BinaryHeap}, io::{BufRead, stdin}};

#[derive(Clone, PartialEq, Eq, Hash)]
struct Voxel {
    x: i16,
    y: i16,
    z: i16
}

impl Voxel {
    fn parse(line: &str) -> Self {
        let (x, y, z) = sscanf!(line, "{},{},{}", i16, i16, i16).unwrap();

        Self { x, y, z }
    }

    fn zero() -> Self {
        Self {
            x: 0,
            y: 0,
            z: 0,
        }
    }

    fn distance_to(&self, other: &Self) -> i16 {
        (self.x - other.x).abs()
            + (self.y - other.y).abs()
            + (self.z - other.z).abs()
    }

    fn sides(&self) -> impl Iterator<Item=Voxel> {
        [
            Voxel { x: self.x - 1, ..*self },
            Voxel { x: self.x + 1, ..*self },
            Voxel { y: self.y - 1, ..*self },
            Voxel { y: self.y + 1, ..*self },
            Voxel { z: self.z - 1, ..*self },
            Voxel { z: self.z + 1, ..*self },
        ].into_iter()
    }
}

struct VoxelDistance(Voxel, i16);

impl VoxelDistance {
    fn new(voxel: Voxel, distance: i16) -> Self {
        Self(voxel, distance)
    }
}

impl Eq for VoxelDistance {
    // pass
}

impl PartialEq for VoxelDistance {
    fn eq(&self, other: &Self) -> bool {
        self.1.eq(&other.1)
    }
}

impl Ord for VoxelDistance {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.1.cmp(&self.1)
    }
}

impl PartialOrd for VoxelDistance {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

struct Voxels {
    voxels: HashSet<Voxel>
}

impl Voxels {
    fn parse_all(reader: impl BufRead) -> Self {
        let voxels = reader.lines()
            .filter_map(|line| line.ok())
            .map(|line| Voxel::parse(&line))
            .collect();

        Self { voxels }
    }

    fn sides<'a>(&'a self) -> impl Iterator<Item=Voxel> + 'a {
        self.voxels.iter()
            .flat_map(|voxel| voxel.sides())
            .filter(|voxel| !self.voxels.contains(&voxel))
    }

    fn is_reachable(&self, starting_point: &Voxel, end_point: &Voxel) -> bool {
        let mut visited = HashSet::new();
        let mut to_visit = BinaryHeap::new();
        to_visit.push(VoxelDistance::new(
            starting_point.clone(),
            starting_point.distance_to(&end_point)
        ));

        while let Some(VoxelDistance(curr, _)) = to_visit.pop() {
            if end_point.eq(&curr) {
                return true;
            }

            for next_voxel in curr.sides() {
                if !self.voxels.contains(&next_voxel) && !visited.contains(&next_voxel) {
                    visited.insert(next_voxel.clone());
                    to_visit.push(VoxelDistance::new(
                        next_voxel.clone(),
                        next_voxel.distance_to(&end_point),
                    ));
                }
            }
        }

        false
    }

    #[cfg(test)]
    fn len(&self) -> usize {
        self.voxels.len()
    }
}

fn main() {
    let stdin = stdin().lock();
    let voxels = Voxels::parse_all(stdin);

    println!("{}", voxels.sides().count());
    println!("{}", voxels.sides().filter(|side| voxels.is_reachable(&side, &Voxel::zero())).count());
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    const EXAMPLE: &str = r#"2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5"#;

    #[test]
    fn _01_example() {
        let voxels = Voxels::parse_all(Cursor::new(EXAMPLE));

        assert_eq!(voxels.len(), 13);
        assert_eq!(voxels.sides().count(), 64);
    }

    #[test]
    fn _02_example() {
        let voxels = Voxels::parse_all(Cursor::new(EXAMPLE));

        assert_eq!(voxels.sides().filter(|side| voxels.is_reachable(&side, &Voxel::zero())).count(), 58);
    }
}