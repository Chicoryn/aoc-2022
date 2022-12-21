use sscanf::sscanf;
use std::{io::{prelude::*, stdin}, collections::HashMap};

#[derive(Hash, PartialEq, Eq)]
enum MonkeyJob {
    Const(i64),
    Eq(String, String),
    Add(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
}

impl MonkeyJob {
    fn lhs(&self) -> String {
        match self {
            Self::Eq(lhs, _) => lhs.clone(),
            Self::Add(lhs, _) => lhs.clone(),
            Self::Sub(lhs, _) => lhs.clone(),
            Self::Mul(lhs, _) => lhs.clone(),
            Self::Div(lhs, _) => lhs.clone(),
            _ => panic!()
        }
    }

    fn rhs(&self) -> String {
        match self {
            Self::Add(_, rhs) => rhs.clone(),
            Self::Eq(_, rhs) => rhs.clone(),
            Self::Sub(_, rhs) => rhs.clone(),
            Self::Mul(_, rhs) => rhs.clone(),
            Self::Div(_, rhs) => rhs.clone(),
            _ => panic!()
        }
    }
}

#[derive(Hash, PartialEq, Eq)]
struct Monkey {
    name: String,
    job: MonkeyJob
}

impl Monkey {
    fn parse(line: &str) -> Self {
        if let Ok((name, lhs, rhs)) = sscanf!(line, "{}: {} + {}", String, String, String) {
            Self::new(name, MonkeyJob::Add(lhs, rhs))
        } else if let Ok((name, lhs, rhs)) = sscanf!(line, "{}: {} - {}", String, String, String) {
            Self::new(name, MonkeyJob::Sub(lhs, rhs))
        } else if let Ok((name, lhs, rhs)) = sscanf!(line, "{}: {} * {}", String, String, String) {
            Self::new(name, MonkeyJob::Mul(lhs, rhs))
        } else if let Ok((name, lhs, rhs)) = sscanf!(line, "{}: {} / {}", String, String, String) {
            Self::new(name, MonkeyJob::Div(lhs, rhs))
        } else if let Ok((name, value)) = sscanf!(line, "{}: {}", String, i64) {
            Self::new(name, MonkeyJob::Const(value))
        } else {
            panic!("unrecognized line -- {}", line)
        }
    }

    fn new(name: String, job: MonkeyJob) -> Self {
        Self { name, job }
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn job(&self) -> &MonkeyJob {
        &self.job
    }

    fn map(&self, mapper: impl Fn(&MonkeyJob) -> MonkeyJob) -> Self {
        Self {
            name: self.name.clone(),
            job: mapper(&self.job),
        }
    }
}

struct Monkeys {
    monkeys: HashMap<String, Monkey>
}

impl Monkeys {
    fn parse_all(reader: impl BufRead) -> Self {
        let monkeys = reader.lines()
            .filter_map(|line| line.ok())
            .map(|line| Monkey::parse(&line))
            .map(|monkey| (monkey.name.clone(), monkey))
            .collect();

        Self { monkeys }
    }

    fn map(&mut self, name: &str, mapper: impl Fn(&MonkeyJob) -> MonkeyJob) -> &Self {
        self.monkeys.entry(name.to_string()).and_modify(|value| {
            *value = value.map(mapper);
        });

        self
    }

    fn lazy_evaluate(&self, monkey: &Monkey, visited: &mut HashMap<String, i64>) -> i64 {
        if let Some(result) = visited.get(monkey.name()) {
            *result
        } else {
            let result = match monkey.job() {
                MonkeyJob::Const(value) => *value,
                MonkeyJob::Eq(lhs, rhs) => (self.lazy_evaluate(&self.monkeys[lhs], visited) == self.lazy_evaluate(&self.monkeys[rhs], visited)) as i64,
                MonkeyJob::Add(lhs, rhs) => self.lazy_evaluate(&self.monkeys[lhs], visited) + self.lazy_evaluate(&self.monkeys[rhs], visited),
                MonkeyJob::Sub(lhs, rhs) => self.lazy_evaluate(&self.monkeys[lhs], visited) - self.lazy_evaluate(&self.monkeys[rhs], visited),
                MonkeyJob::Mul(lhs, rhs) => self.lazy_evaluate(&self.monkeys[lhs], visited) * self.lazy_evaluate(&self.monkeys[rhs], visited),
                MonkeyJob::Div(lhs, rhs) => self.lazy_evaluate(&self.monkeys[lhs], visited) / self.lazy_evaluate(&self.monkeys[rhs], visited),
            };

            visited.insert(monkey.name().to_string(), result);
            result
        }
    }

    fn contains(&self, root: &str, element: &str) -> bool {
        let monkey = &self.monkeys[root];

        monkey.name() == element || match monkey.job() {
            MonkeyJob::Const(_) => false,
            _ => self.contains(&monkey.job().lhs(), element) || self.contains(&monkey.job().rhs(), element)
        }
    }

    fn evaluate(&self, name: &str) -> i64 {
        self.lazy_evaluate(&self.monkeys[name], &mut HashMap::new())
    }

    fn backward(&self, start_at: &str, start_value: i64, name: &str) -> i64 {
        if start_at == name {
            start_value
        } else {
            match self.monkeys[start_at].job() {
                MonkeyJob::Const(_) => panic!(),
                MonkeyJob::Eq(lhs, rhs) if self.contains(&lhs, name) => self.backward(&lhs, self.evaluate(&rhs), name),
                MonkeyJob::Eq(lhs, rhs) if self.contains(&rhs, name) => self.backward(&rhs,  self.evaluate(&lhs), name),
                MonkeyJob::Add(lhs, rhs) if self.contains(&lhs, name) => self.backward(&lhs, start_value - self.evaluate(&rhs), name),
                MonkeyJob::Add(lhs, rhs) if self.contains(&rhs, name) => self.backward(&rhs, start_value - self.evaluate(&lhs), name),
                MonkeyJob::Sub(lhs, rhs) if self.contains(&lhs, name) => self.backward(&lhs, start_value + self.evaluate(&rhs), name),
                MonkeyJob::Sub(lhs, rhs) if self.contains(&rhs, name) => self.backward(&rhs, self.evaluate(&lhs) - start_value, name),
                MonkeyJob::Mul(lhs, rhs) if self.contains(&lhs, name) => self.backward(&lhs, start_value / self.evaluate(&rhs), name),
                MonkeyJob::Mul(lhs, rhs) if self.contains(&rhs, name) => self.backward(&rhs, start_value / self.evaluate(&lhs), name),
                MonkeyJob::Div(lhs, rhs) if self.contains(&lhs, name) => self.backward(&lhs, start_value * self.evaluate(&rhs), name),
                MonkeyJob::Div(lhs, rhs) if self.contains(&rhs, name) => self.backward(&rhs, self.evaluate(&lhs) / start_value, name),
                _ => panic!("could not find {} in {}", name, start_at)
            }
        }
    }
}

fn main() {
    let stdin = stdin().lock();
    let mut monkeys = Monkeys::parse_all(stdin);

    println!("{}", monkeys.evaluate("root")); // 276156919469632
    println!("{}", monkeys.map("root", |job| MonkeyJob::Eq(job.lhs(), job.rhs())).backward("root", 1, "humn"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    const EXAMPLE: &str = r#"root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32"#;

    #[test]
    fn _01_example() {
        let monkeys = Monkeys::parse_all(Cursor::new(EXAMPLE));
        assert_eq!(monkeys.evaluate("root"), 152);
    }

    #[test]
    fn _02_example() {
        let mut monkeys = Monkeys::parse_all(Cursor::new(EXAMPLE));

        assert_eq!(monkeys.map("root", |job| MonkeyJob::Eq(job.lhs(), job.rhs())).backward("root", 1, "humn"), 301);
    }
}