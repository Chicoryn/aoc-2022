use ndarray::prelude::*;
use sscanf::sscanf;
use std::{collections::VecDeque, io::{prelude::*, stdin}, fmt::Debug};

struct Valve {
    name: String,
    flow_rate: u32,
    leads_to: Vec<String>,
    leads_to_indices: Vec<usize>,
}

impl Valve {
    fn parse(line: &str) -> Self {
        let (name, flow_rate, leads_to) =
            if let Ok((name, flow_rate, leads_to)) = sscanf!(line, "Valve {} has flow rate={}; tunnels lead to valves {}", String, u32, String) {
                (name, flow_rate, leads_to)
            } else if let Ok((name, flow_rate, leads_to)) = sscanf!(line, "Valve {} has flow rate={}; tunnel leads to valve {}", String, u32, String) {
                (name, flow_rate, leads_to)
            } else {
                panic!();
            };

        Self {
            name,
            flow_rate,
            leads_to: leads_to.split(",").map(|part| part.trim().to_string()).collect(),
            leads_to_indices: vec! []
        }
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn flow_rate(&self) -> u32 {
        self.flow_rate
    }

    fn leads_to(&self) -> &[usize] {
        &self.leads_to_indices
    }

    fn with_valves(&self, valves: &[Valve]) -> Self {
        Self {
            name: self.name.clone(),
            flow_rate: self.flow_rate,
            leads_to: self.leads_to.clone(),
            leads_to_indices: self.leads_to.iter().map(|other_name| {
                valves.iter().position(|other_valve| other_valve.name() == other_name).unwrap()
            }).collect(),
        }
    }
}

#[derive(Clone)]
struct Path {
    opened: u64,
    mins_remaining: u32,
    at: usize,
    points: u32
}

impl Path {
    fn starting_point(mins_remaining: u32) -> Self {
        Self {
            opened: 0,
            mins_remaining,
            at: 0,
            points: 0
        }
    }

    fn open(&self, to_open: usize, distance_to: u32, flow_rate: u32) -> Self {
        let mins_remaining = self.mins_remaining - distance_to - 1;
        let opened = self.opened | (1 << to_open);

        Self {
            opened,
            mins_remaining,
            at: to_open,
            points: self.points + flow_rate * mins_remaining
        }
    }

    fn has_opened(&self, other_valve: usize) -> bool {
        self.opened & (1 << other_valve) > 0
    }
}

impl Debug for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:?} ({})", self.opened, self.points)
    }
}

struct Valves {
    valves: Vec<Valve>
}

impl Valves {
    fn parse_all<R: BufRead>(reader: R) -> Self {
        let mut valves = reader.lines()
            .filter_map(|line| line.ok())
            .map(|line| Valve::parse(&line))
            .collect::<Vec<_>>();
        valves.sort_by_key(|valve| valve.name().to_string());

        Self {
            valves: valves.iter()
                .map(|valve| valve.with_valves(&valves))
                .collect::<Vec<_>>()
        }
    }

    fn distance_matrix(&self) -> Array2<u32> {
        let n = self.valves.len();
        let mut shortest_so_far = Array2::from_elem((n, n), u32::MAX);

        for i in 0..n {
            let mut to_visit = VecDeque::new();
            to_visit.push_back(i);
            shortest_so_far[(i,i)] = 0;

            while let Some(j) = to_visit.pop_front() {
                let curr_distance = shortest_so_far[(i,j)];

                for &k in self.valves[j].leads_to() {
                    if shortest_so_far[(i,k)] > curr_distance + 1 {
                        shortest_so_far[(i,k)] = curr_distance + 1;
                        to_visit.push_back(k);
                    }
                }
            }
        }

        shortest_so_far
    }

    fn max_flow_path_aux(
        &self,
        actors: usize,
        in_mins: u32,
        distances: &Array2<u32>,
        exclude: u64
    ) -> u32
    {
        let nz_valves = self.valves.iter()
            .enumerate()
            .filter_map(|(i, valve)| {
                if valve.flow_rate() > 0 && (exclude & (1 << i)) == 0 {
                    Some(i)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        let mut to_visit = VecDeque::new();
        let mut so_far = u32::MIN;
        to_visit.push_back(Path::starting_point(in_mins));

        while let Some(path) = to_visit.pop_front() {
            let path = &path;
            let remaining_valves = nz_valves.iter()
                .filter(|&&nz_valve| distances[(path.at, nz_valve)] < path.mins_remaining)
                .filter(|&&nz_valve| !path.has_opened(nz_valve));

            for &nz_valve in remaining_valves {
                to_visit.push_back(path.open(
                    nz_valve,
                    distances[(path.at, nz_valve)],
                    self.valves[nz_valve].flow_rate()
                ));
            }

            let points_with_actors = if actors > 1 {
                path.points + self.max_flow_path_aux(
                    actors - 1,
                    in_mins,
                    distances,
                    exclude | path.opened
                )
            } else {
                path.points
            };

            if points_with_actors > so_far {
                so_far = so_far.max(points_with_actors);
            }
        }

        so_far
    }

    fn max_flow_path(&self, actors: usize, in_mins: u32) -> u32 {
        let distances = self.distance_matrix();

        self.max_flow_path_aux(actors, in_mins, &distances, 0)
    }
}

fn main() {
    let stdin = stdin().lock();
    let valves = Valves::parse_all(stdin);

    println!("{}", valves.max_flow_path(1, 30));
    println!("{}", valves.max_flow_path(2, 26));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    const EXAMPLE: &str = r#"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II"#;

    #[test]
    fn _01_example() {
        let valves = Valves::parse_all(Cursor::new(EXAMPLE));
        assert_eq!(valves.max_flow_path(1, 30), 1651);
    }

    #[test]
    fn _02_example() {
        let valves = Valves::parse_all(Cursor::new(EXAMPLE));
        assert_eq!(valves.max_flow_path(2, 26), 1707);
    }
}
