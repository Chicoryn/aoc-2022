use sscanf::sscanf;
use core::panic;
use std::io::{prelude::*, self, stdin};

#[derive(Clone, Debug)]
enum Op {
    Add { rhs: isize },
    Mul { rhs: isize },
    Mod { rhs: isize },
    Sq,
}

impl Op {
    fn execute(&self, lhs: isize) -> isize {
        match *self {
            Self::Add { rhs } => lhs + rhs,
            Self::Mul { rhs } => lhs * rhs,
            Self::Mod { rhs } => lhs % rhs,
            Self::Sq => lhs * lhs
        }
    }
}

#[derive(Clone, Debug)]
struct Test {
    check: Op,
    if_true: usize,
    if_false: usize,
}

impl Test {
    fn target_monkey(&self, worry_level: isize) -> usize {
        if self.check.execute(worry_level) == 0 {
            self.if_true
        } else {
            self.if_false
        }
    }
}

#[derive(Clone, Debug)]
struct Monkey {
    inspected_items: usize,
    items: Vec<isize>,
    operation: Op,
    test: Test
}

impl Monkey {
    fn parse<R: BufRead>(reader: &mut R) -> Result<Self, io::Error> {
        let mut operation = None;
        let mut test = None;
        let mut items = vec! [];

        for line in reader.lines().filter_map(|line| line.ok()) {
            if let Ok(_) = sscanf!(line, "Monkey {}:", usize) {
                // pass
            } else if let Ok(starting_items) = sscanf!(line, "  Starting items: {}", String) {
                    for worry_level in starting_items.split(",").map(|item| item.trim()) {
                    items.push(worry_level.parse::<isize>().unwrap());
                }
            } else if let Ok(rhs) = sscanf!(line, "  Test: divisible by {}", isize) {
                test = Some(Test {
                    check: Op::Mod { rhs },
                    if_true: 0,
                    if_false: 0,
                });
            } else if let Ok(if_true) = sscanf!(line, "    If true: throw to monkey {}", usize) {
                test = test.map(|mut test| {
                    test.if_true = if_true;
                    test
                });
            } else if let Ok(if_false) = sscanf!(line, "    If false: throw to monkey {}", usize) {
                test = test.map(|mut test| {
                    test.if_false = if_false;
                    test
                });
            } else if let Ok(rhs) = sscanf!(line, "  Operation: new = old * {}", isize) {
                operation = Some(Op::Mul { rhs });
            } else if let Ok(_) = sscanf!(line, "  Operation: new = old * old") {
                operation = Some(Op::Sq);
            } else if let Ok(rhs) = sscanf!(line, "  Operation: new = old + {}", isize) {
                operation = Some(Op::Add { rhs });
            } else if let Ok(rhs) = sscanf!(line, "  Operation: new = old - {}", isize) {
                operation = Some(Op::Add { rhs: -rhs });
            } else if line.is_empty() {
                break
            }
        }

        Ok(Self {
            inspected_items: 0,
            items,
            operation: operation.ok_or(io::ErrorKind::NotFound)?,
            test: test.ok_or(io::ErrorKind::NotFound)?,
        })
    }

    fn parse_all<R: BufRead>(mut reader: R) -> Vec<Self> {
        let mut monkeys = vec! [];

        while let Ok(monkey) = Self::parse(&mut reader) {
            monkeys.push(monkey);
        }

        monkeys
    }

    fn safe_modulus(&self) -> isize {
        match self.test.check {
            Op::Mod { rhs } => rhs,
            _ => panic!()
        }
    }

    fn drain_items(&mut self, relief: &impl Fn(isize) -> isize) -> Vec<(isize, usize)> {
        self.inspected_items += self.items.len();
        self.items.drain(..).map(|worry_level| {
            let new_worry_level = relief(self.operation.execute(worry_level));

            (new_worry_level, self.test.target_monkey(new_worry_level))
        }).collect()
    }

    fn push(&mut self, worry_level: isize) {
        self.items.push(worry_level);
    }

    fn inspected_items(&self) -> usize {
        self.inspected_items
    }
}

fn execute_round(monkeys: &mut [Monkey], relief: impl Fn(isize) -> isize) {
    for i in 0..monkeys.len() {
        for (worry_level, to_monkey) in monkeys[i].drain_items(&relief) {
            monkeys[to_monkey].push(worry_level);
        }
    }
}

fn monkey_business(mut inspected_items: Vec<usize>) -> usize {
    inspected_items.sort();

    if inspected_items.len() > 2 {
        let n = inspected_items.len();

        inspected_items[n - 1] * inspected_items[n - 2]
    } else {
        0
    }
}

fn main() {
    let stdin = stdin().lock();
    let mut monkeys1 = Monkey::parse_all(stdin);
    let mut monkeys2 = monkeys1.clone();
    let total_mod = monkeys2.iter().map(|monkey| monkey.safe_modulus()).product::<isize>();
    for _ in 0..20 { execute_round(&mut monkeys1, |worry_level| worry_level / 3); }
    for _ in 0..10000 { execute_round(&mut monkeys2, |worry_level| worry_level % total_mod); }

    println!("{}", monkey_business(monkeys1.iter().map(|monkey| monkey.inspected_items()).collect()));
    println!("{}", monkey_business(monkeys2.iter().map(|monkey| monkey.inspected_items()).collect()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    const EXAMPLE: &str = r#"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1"#;

    #[test]
    fn _01_example() {
        let mut monkeys = Monkey::parse_all(Cursor::new(EXAMPLE));
        for _ in 0..20 { execute_round(&mut monkeys, |worry_level| worry_level / 3); }

        assert_eq!(monkeys.len(), 4);
        assert_eq!(monkeys[0].items, vec! [10, 12, 14, 26, 34]);
        assert_eq!(monkeys[1].items, vec! [245, 93, 53, 199, 115]);
        assert_eq!(monkeys[2].items, vec! []);
        assert_eq!(monkeys[3].items, vec! []);
        assert_eq!(monkey_business(monkeys.iter().map(|monkey| monkey.inspected_items()).collect()), 10605);
    }

    #[test]
    fn _02_example() {
        let mut monkeys = Monkey::parse_all(Cursor::new(EXAMPLE));
        let total_mod = monkeys.iter().map(|monkey| monkey.safe_modulus()).product::<isize>();
        for _ in 0..10000 { execute_round(&mut monkeys, |worry_level| worry_level % total_mod); }

        assert_eq!(monkeys.len(), 4);
        assert_eq!(monkeys[0].inspected_items(), 52166);
        assert_eq!(monkeys[1].inspected_items(), 47830);
        assert_eq!(monkeys[2].inspected_items(), 1938);
        assert_eq!(monkeys[3].inspected_items(), 52013);
        assert_eq!(monkey_business(monkeys.iter().map(|monkey| monkey.inspected_items()).collect()), 2713310158);
    }
}
