use std::hash::Hash;
use std::io::{prelude::*, stdin};
use std::ops::{AddAssign, SubAssign, Mul};
use sscanf::sscanf;

#[derive(Copy, Clone, Debug)]
struct Resources {
    ore: u16,
    clay: u16,
    obsidian: u16,
    geode: u16,
}

impl Resources {
    fn new(ore: u16, clay: u16, obsidian: u16, geode: u16) -> Self {
        Self { ore, clay, obsidian, geode }
    }

    fn ore(&self) -> u16 {
        self.ore
    }

    fn clay(&self) -> u16 {
        self.clay
    }

    fn obsidian(&self) -> u16 {
        self.obsidian
    }

    fn geode(&self) -> u16 {
        self.geode
    }

    fn max(&self) -> u16 {
        self.ore
            .max(self.clay)
            .max(self.obsidian)
            .max(self.geode)
    }

    fn saturating_sub(&self, rhs: &Self) -> Self {
        Self {
            ore: self.ore.saturating_sub(rhs.ore),
            clay: self.clay.saturating_sub(rhs.clay),
            obsidian: self.obsidian.saturating_sub(rhs.obsidian),
            geode: self.geode.saturating_sub(rhs.geode),
        }
    }

    fn div_ceil(&self, rhs: &Self) -> Self {
        Self {
            ore: if rhs.ore > 0 { (self.ore + rhs.ore - 1) / rhs.ore } else { self.ore },
            clay: if rhs.clay > 0 { (self.clay + rhs.clay - 1) / rhs.clay } else { self.clay },
            obsidian: if rhs.obsidian > 0 { (self.obsidian + rhs.obsidian - 1) / rhs.obsidian } else { self.obsidian },
            geode: if rhs.geode > 0 { (self.geode + rhs.geode - 1) / rhs.geode } else { self.geode },
        }
    }
}

impl AddAssign for Resources {
    fn add_assign(&mut self, rhs: Self) {
        self.ore += rhs.ore;
        self.clay += rhs.clay;
        self.obsidian += rhs.obsidian;
        self.geode += rhs.geode;
    }
}

impl SubAssign for Resources {
    fn sub_assign(&mut self, rhs: Self) {
        self.ore -= rhs.ore;
        self.clay -= rhs.clay;
        self.obsidian -= rhs.obsidian;
        self.geode -= rhs.geode;
    }
}

impl Mul<u16> for Resources {
    type Output = Self;

    fn mul(self, rhs: u16) -> Self {
        Self {
            ore: self.ore * rhs,
            clay: self.clay * rhs,
            obsidian: self.obsidian * rhs,
            geode: self.geode * rhs
        }
    }
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
enum Robot {
    Ore = 0,
    Clay = 1,
    Obsidian = 2,
    Geode = 3
}

impl Robot {
    fn all() -> impl Iterator<Item=Robot> {
        [
            Self::Geode,
            Self::Obsidian,
            Self::Clay,
            Self::Ore,
        ].into_iter()
    }
}

#[derive(Clone, Debug)]
struct Blueprint {
    id: usize,
    costs: [Resources; 4],
    max: Resources
}

impl Blueprint {
    fn parse(line: &str) -> Self {
        let (id, ore_ore_cost, clay_ore_cost, obsidian_ore_cost, obsidian_clay_cost, geode_ore_cost, geode_obsidian_cost) = sscanf!(line, "Blueprint {}: Each ore robot costs {} ore. Each clay robot costs {} ore. Each obsidian robot costs {} ore and {} clay. Each geode robot costs {} ore and {} obsidian.", usize, u16, u16, u16, u16, u16, u16).unwrap();

        Self {
            id,
            costs: [
                Resources::new(ore_ore_cost, 0, 0, 0),
                Resources::new(clay_ore_cost, 0, 0, 0),
                Resources::new(obsidian_ore_cost, obsidian_clay_cost, 0, 0),
                Resources::new(geode_ore_cost, 0, geode_obsidian_cost, 0),
            ],
            max: Resources::new(
                ore_ore_cost.max(clay_ore_cost).max(obsidian_ore_cost).max(geode_ore_cost),
                obsidian_clay_cost,
                geode_obsidian_cost,
                0,
            )
        }
    }

    fn cost(&self, robot: Robot) -> &Resources {
        &self.costs[robot as usize]
    }

    fn max(&self) -> &Resources {
        &self.max
    }
}

#[derive(Debug)]
struct FactoryPlan {
    to_build: Robot,
    time: usize,
}

#[derive(Clone, Debug)]
struct Factory<'a> {
    blueprint: &'a Blueprint,
    remaining_time: usize,
    resources: Resources,
    robots: Resources,
}

impl<'a> Factory<'a> {
    fn new(blueprint: &'a Blueprint, remaining_time: usize) -> Self {
        Self {
            blueprint,
            remaining_time,
            resources: Resources::new(0, 0, 0, 0),
            robots: Resources::new(1, 0, 0, 0),
        }
    }

    fn score(&self) -> usize {
        self.resources.geode() as usize
    }

    fn relax(&self) -> usize {
        let production = self.robots.geode() as usize;
        let effective_time = self.remaining_time.saturating_sub(1);

        self.score()
            + production * self.remaining_time
            + (effective_time * (effective_time + 1)) / 2
    }

    fn is_buildable(&self, cost: &Resources) -> bool {
        (cost.ore() == 0 || self.robots.ore() > 0)
            && (cost.clay() == 0 || self.robots.clay() > 0)
            && (cost.obsidian() == 0 || self.robots.obsidian() > 0)
    }

    fn makes_sense(&self, robot: &Robot) -> bool {
        match robot {
            Robot::Geode => true,
            Robot::Obsidian => self.robots.obsidian() < self.blueprint.max().obsidian(),
            Robot::Clay => self.robots.clay() < self.blueprint.max().clay(),
            Robot::Ore => self.robots.ore() < self.blueprint.max().ore(),
        }
    }

    fn make_plan(&self, robot: &Robot, cost: &Resources) -> FactoryPlan {
        let remaining_cost = cost.saturating_sub(&self.resources);
        let remaining_turns = remaining_cost.div_ceil(&self.robots);
        let time = remaining_turns.max() as usize + 1;

        FactoryPlan {
            to_build: *robot,
            time
        }
    }

    fn plans<'b>(&'b self) -> impl Iterator<Item=FactoryPlan> + 'b {
        Robot::all()
            .map(|robot| (robot, self.blueprint.cost(robot)))
            .filter(|(robot, cost)| self.makes_sense(robot) && self.is_buildable(cost))
            .map(|(robot, cost)| self.make_plan(&robot, cost))
            .filter(|plan| plan.time <= self.remaining_time)
    }

    fn next_step(&self, plan: &FactoryPlan) -> Self {
        let mut resources = self.resources.clone();
        resources += self.robots * plan.time as u16;
        resources -= *self.blueprint.cost(plan.to_build);

        let mut robots = self.robots.clone();
        robots += match plan.to_build {
            Robot::Ore => { Resources::new(1, 0, 0, 0) },
            Robot::Clay => { Resources::new(0, 1, 0, 0) },
            Robot::Obsidian => { Resources::new(0, 0, 1, 0) },
            Robot::Geode => { Resources::new(0, 0, 0, 1) },
        };

        Self {
            blueprint: self.blueprint,
            remaining_time: self.remaining_time.saturating_sub(plan.time),
            resources,
            robots
        }
    }
}

fn largest_geode_count(blueprint: &Blueprint, remaining_time: usize) -> usize {
    let mut so_far = usize::MIN;
    let mut to_visit = Vec::new();
    to_visit.push(Factory::new(&blueprint, remaining_time));

    while let Some(state) = to_visit.pop() {
        so_far = so_far.max(state.score());

        for plan in state.plans() {
            let next_state = state.next_step(&plan);

            if next_state.relax() > so_far {
                to_visit.push(next_state);
            }
        }
    }

    so_far
}

struct Blueprints {
    blueprints: Vec<Blueprint>
}

impl Blueprints {
    fn parse_all(reader: impl BufRead) -> Self {
        Self {
            blueprints: reader.lines()
                .filter_map(|line| line.ok())
                .map(|line| Blueprint::parse(&line))
                .collect()
        }
    }

    fn take(&self, n: usize) -> Self {
        Self {
            blueprints: self.blueprints[0..n.min(self.blueprints.len())].to_vec()
        }
    }

    fn total_quality_level(&self, remaining_time: usize) -> usize {
        self.blueprints.iter()
            .map(|blueprint| blueprint.id * largest_geode_count(blueprint, remaining_time))
            .sum()
    }

    fn geode_product(&self, remaining_time: usize) -> usize {
        self.blueprints.iter()
            .map(|blueprint| largest_geode_count(blueprint, remaining_time))
            .product()
    }
}

fn main() {
    let stdin = stdin().lock();
    let blueprints = Blueprints::parse_all(stdin);

    println!("{}", blueprints.total_quality_level(24));
    println!("{}", blueprints.take(3).geode_product(32));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    const EXAMPLE: &str = r#"Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian."#;

    #[test]
    fn _01_example() {
        let blueprints = Blueprints::parse_all(Cursor::new(EXAMPLE));
        assert_eq!(blueprints.total_quality_level(24), 33);
    }

    #[test]
    fn _02_example() {
        let blueprints = Blueprints::parse_all(Cursor::new(EXAMPLE));
        assert_eq!(blueprints.take(3).geode_product(32), 3472);
    }
}