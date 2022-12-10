use sscanf::sscanf;
use std::io::{prelude::*, stdin};

#[derive(Clone, Copy)]
enum Instruction {
    AddX(isize),
    Noop
}

impl Instruction {
    fn as_microcode(self) -> Vec<Instruction> {
        match self {
            Self::Noop => vec! [Self::Noop],
            Self::AddX(amount) => vec![Self::Noop, Self::AddX(amount)]
        }
    }
}

struct CPU {
    cycle: usize,
    register_value: Vec<isize>,
    current_value: isize
}

impl CPU {
    fn new() -> Self {
        Self {
            cycle: 0,
            register_value: vec! [1],
            current_value: 1,
        }
    }

    #[cfg(test)]
    fn current_cycle(&self) -> usize {
        self.cycle
    }

    fn current_value(&self) -> isize {
        self.current_value
    }

    fn cycles<'a>(&'a self) -> impl Iterator<Item=isize> + 'a {
        self.register_value.iter().cloned()
    }

    fn execute_microcode(&mut self, inst: Instruction) {
        self.cycle += 1;
        self.register_value.push(self.current_value());
        self.current_value += match inst {
            Instruction::Noop => 0,
            Instruction::AddX(amount) => amount
        };
    }

    fn execute(&mut self, inst: Instruction) {
        for &mc_inst in inst.as_microcode().iter() {
            self.execute_microcode(mc_inst);
        }
    }
}

struct Program {
    cpu: CPU
}

impl Program {
    fn parse_all<R: BufRead>(reader: R) -> Self {
        let mut cpu = CPU::new();

        for line in reader.lines().filter_map(|line| line.ok()) {
            if let Ok(_) = sscanf!(line, "noop") {
                cpu.execute(Instruction::Noop);
            } else if let Ok(amount) = sscanf!(line, "addx {}", isize) {
                cpu.execute(Instruction::AddX(amount));
            }
        }

        Self { cpu }
    }

    fn cycles<'a>(&'a self) -> impl Iterator<Item=isize> + 'a {
        self.cpu.cycles()
    }

    #[cfg(test)]
    fn current_cycle(&self) -> usize {
        self.cpu.current_cycle()
    }

    #[cfg(test)]
    fn current_value(&self) -> isize {
        self.cpu.current_value()
    }

    fn screen(&self) -> String {
        let mut output = String::new();

        for (i, signal) in self.cycles().skip(1).enumerate() {
            let position = (i % 40) as isize;

            if i > 0 && position == 0 {
                output += "\n";
            }

            output += if signal >= position - 1 && signal <= position + 1 {
                "#"
            } else {
                "."
            };
        }

        output
    }
}


fn main() {
    let stdin = stdin().lock();
    let prog = Program::parse_all(stdin);

    println!("{}", prog.cycles().enumerate().skip(20).step_by(40).map(|(cycle, signal_strength)| cycle as isize * signal_strength).sum::<isize>());
    println!("{}", prog.screen());
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    const EXAMPLE: &str = r#"noop
addx 3
addx -5"#;

    const LARGE_EXAMPLE: &str = r#"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"#;

    #[test]
    fn _01_example() {
        let prog = Program::parse_all(Cursor::new(EXAMPLE));

        assert_eq!(prog.current_value(), -1);
        assert_eq!(prog.current_cycle(), 5);
    }

    #[test]
    fn _01_large_example() {
        let prog = Program::parse_all(Cursor::new(LARGE_EXAMPLE));

        assert_eq!(prog.cycles().enumerate().skip(20).step_by(40).map(|(cycle, signal_strength)| cycle as isize * signal_strength).sum::<isize>(), 13140);
    }

    #[test]
    fn _02_example() {
        let prog = Program::parse_all(Cursor::new(LARGE_EXAMPLE));

        assert_eq!(prog.screen(), r#"##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."#);
    }
}