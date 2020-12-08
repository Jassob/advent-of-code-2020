use std::collections::HashSet;

fn main() -> Result<(), String> {
    let (part, content) = utils::parse_args()?;
    let instructions: Vec<(String, i32)> = content
        .lines()
        .map(parse_line)
        .collect::<Result<Vec<(String, i32)>, String>>()?;

    utils::run(part1, part2, part, instructions);
    Ok(())
}

fn part1(instrs: Vec<(String, i32)>) -> i32 {
    let mut m = Machine::new();
    let mut executed_instrs: HashSet<usize> = HashSet::new();
    while !executed_instrs.contains(&m.pc) {
        executed_instrs.insert(m.pc);
        m.step(&instrs);
    }
    m.acc
}

fn part2(instr: Vec<(String, i32)>) -> i32 {
    unimplemented!()
}

fn parse_line(l: &str) -> Result<(String, i32), String> {
    let mut words = l.split(' ');
    let instr = words.next().map_or_else(
        || Err("missing instruction".to_owned()),
        |i| Ok(i.to_string()),
    )?;
    let operand = words.next().map_or_else(
        || Err("missing operand".to_owned()),
        |o| {
            let sign = o.chars().next();
            o.chars()
                .skip(1)
                .collect::<String>()
                .parse::<i32>()
                .map(|n| if sign == Some('-') { -n } else { n })
                .map_err(|e| format!("failed to parse operand as string: {}", e))
        },
    )?;
    Ok((instr, operand))
}

#[derive(Debug, Clone)]
struct Machine {
    pc: usize,
    acc: i32,
}

impl Machine {
    fn new() -> Machine {
        Machine { pc: 0, acc: 0 }
    }
    fn step(&mut self, instrs: &Vec<(String, i32)>) {
        let (instr, o) = &instrs[self.pc];
        match instr.as_ref() {
            "acc" => {
                self.acc += *o;
                self.pc += 1
            }
            "nop" => self.pc += 1,
            "jmp" => {
                let new_pc = self.pc as i64 + *o as i64;
                if new_pc < 0 || new_pc as usize > instrs.len() {
                    panic!(
                        "jmp would result in a out-of-bounds jump ({} + {} > {})",
                        self.pc,
                        o,
                        instrs.len()
                    );
                } else {
                    self.pc = new_pc as usize;
                }
            }
            i => panic!("found unknown instruction: {} with operand {}", i, o),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    #[test]
    fn test_part_one_test_input() {
        let instrs: Vec<(String, i32)> = TEST_INPUT
            .lines()
            .map(parse_line)
            .collect::<Result<Vec<(String, i32)>, String>>()
            .expect("failed to parse test string");
        assert_eq!(part1(instrs), 5);
    }
}
