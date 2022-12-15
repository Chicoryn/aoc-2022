use sscanf::sscanf;
use std::{io::{prelude::*, stdin}, ops::{Bound, RangeBounds}};
use btree_range_map::{RangeSet, AnyRange};

struct Sensor {
    position: (i64, i64),
    beacon: (i64, i64),
}

impl Sensor {
    fn parse_all<R: BufRead>(reader: R) -> Vec<Self> {
        reader.lines()
            .filter_map(|line| line.ok())
            .map(|line| Self::parse(&line))
            .collect()
    }

    fn parse(line: &str) -> Self {
        let (sx, sy, bx, by) = sscanf!(line, "Sensor at x={}, y={}: closest beacon is at x={}, y={}", i64, i64, i64, i64).unwrap();

        Self {
            position: (sx, sy),
            beacon: (bx, by),
        }
    }

    fn closest_beacon(&self) -> (i64, i64) {
        self.beacon
    }

    fn distance_to(&self, other_point: (i64, i64)) -> i64 {
        (other_point.0 - self.position.0).abs() + (other_point.1 - self.position.1).abs()
    }

    fn max_sensor_range(&self) -> i64 {
        self.distance_to(self.closest_beacon())
    }
}

struct Sensors {
    sensors: Vec<Sensor>
}

impl Sensors {
    fn new(sensors: Vec<Sensor>) -> Self {
        Self { sensors }
    }

    fn viable_ys(&self, min: i64, max: i64) -> impl Iterator<Item=i64> {
        let mut borders = vec! [];

        for (i, sensor_0) in self.sensors.iter().enumerate() {
            for sensor_1 in self.sensors.iter().skip(i + 1) {
                if sensor_0.distance_to(sensor_1.position) == sensor_0.max_sensor_range() + sensor_1.max_sensor_range() + 2 {
                    if sensor_0.position.1 < sensor_1.position.1 {
                        borders.push(
                            sensor_0.position.1.max(min).max(sensor_1.position.1 - sensor_1.max_sensor_range())
                            ..=
                            sensor_1.position.1.min(max).min(sensor_0.position.1 + sensor_0.max_sensor_range())
                        );
                    } else {
                        borders.push(
                            sensor_1.position.1.max(min).max(sensor_0.position.1 - sensor_0.max_sensor_range())
                            ..=
                            sensor_0.position.1.min(max).min(sensor_1.position.1 + sensor_1.max_sensor_range())
                        );
                    }
                }
            }
        }

        borders.into_iter().flat_map(|iter| iter)
    }

    fn distress_beacon(&self, min: (i64, i64), max: (i64, i64)) -> (i64, i64) {
        let x_range = AnyRange {
            start: Bound::Included(min.0),
            end: Bound::Included(max.0),
        };

        for y in self.viable_ys(min.1, max.1) {
            let reachable = self.reachable_at_y(y);

            for gap in reachable.complement().iter().filter(|&gap| gap.intersects(&x_range)) {
                return (match gap.start_bound() {
                    Bound::Unbounded => panic!(),
                    Bound::Excluded(&i) => i + 1,
                    Bound::Included(&i) => i,
                }, y)
            }
        }

        panic!()
    }

    fn reachable_at_y(&self, fixed_y: i64) -> RangeSet<i64> {
        let mut visited = RangeSet::new();

        for sensor in &self.sensors {
            let closest_point = (sensor.position.0, fixed_y);
            let base_distance = sensor.distance_to(closest_point);
            let max_distance = sensor.max_sensor_range();
            let x = closest_point.0 as i64;

            if base_distance <= max_distance {
                let n: i64 = max_distance.saturating_sub(base_distance) as i64;
                visited.insert((x - n)..=(x + n));
            }
        }

        visited
    }

    fn reachable_at_y_without_sensors(&self, fixed_y: i64) -> RangeSet<i64> {
        let mut visited = self.reachable_at_y(fixed_y);

        for sensor in self.sensors.iter().filter(|sensor| sensor.closest_beacon().1 == fixed_y) {
            visited.remove(sensor.closest_beacon().0 as i64);
        }

        visited
    }
}

fn main() {
    let stdin = stdin().lock();
    let sensors = Sensors::new(Sensor::parse_all(stdin));
    let beacon_position = sensors.distress_beacon((0, 0), (4000000, 4000000));

    println!("{}", sensors.reachable_at_y_without_sensors(2000000).len());
    println!("{}", beacon_position.0 * 4000000 + beacon_position.1);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    const EXAMPLE: &str = r#"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3"#;

    #[test]
    fn _01_example() {
        let sensors = Sensors::new(Sensor::parse_all(Cursor::new(EXAMPLE)));
        assert_eq!(sensors.reachable_at_y_without_sensors(10).len(), 26);
    }

    #[test]
    fn _02_example() {
        let sensors = Sensors::new(Sensor::parse_all(Cursor::new(EXAMPLE)));
        assert_eq!(sensors.distress_beacon((0, 0), (20, 20)), (14, 11));
    }
}
