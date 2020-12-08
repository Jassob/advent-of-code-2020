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
    match run_instructions(instrs) {
        Err(acc) => acc,
        Ok(_) => panic!("part1 is not supposed to halt, it should loop infinitily."),
    }
}

fn part2(instrs: Vec<(String, i32)>) -> i32 {
    let jmps_or_nops = &instrs
        .iter()
        .filter(|(o, _)| o == "nop" || o == "jmp")
        .count();
    for i in 0..*jmps_or_nops {
        println!("mutating jmp or nop instruction {}", i);
        let new_instrs = mutate(instrs.clone(), i as u32);
        match run_instructions(new_instrs) {
            Err(_) => continue,
            Ok(acc) => return acc,
        }
    }
    panic!("could not create a instruction list that terminated");
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

fn mutate(instrs: Vec<(String, i32)>, instr: u32) -> Vec<(String, i32)> {
    let mut skipped = 0;
    instrs
        .iter()
        .map(|(i, o)| {
            let mut i = i.clone();
            if i.as_str() == "jmp" {
                if skipped == instr {
                    i.clear();
                    i.push_str("nop");
                    println!("replaced jmp with nop");
                }
                skipped += 1;
            } else if i.as_str() == "nop" {
                if skipped == instr {
                    i.clear();
                    i.push_str("jmp");
                    println!("replaced nop with jmp");
                }
                skipped += 1;
            }

            (i, *o)
        })
        .collect()
}

#[derive(Debug, Clone)]
struct Machine {
    pc: usize,
    acc: i32,
    has_halted: bool,
}

impl Machine {
    fn new() -> Machine {
        Machine {
            pc: 0,
            acc: 0,
            has_halted: false,
        }
    }
    fn step(&mut self, instrs: &Vec<(String, i32)>) {
        if self.has_halted {
            return;
        }
        if self.pc == instrs.len() {
            self.has_halted = true;
            return;
        }
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

fn run_instructions(instrs: Vec<(String, i32)>) -> Result<i32, i32> {
    let mut m = Machine::new();
    let mut executed_instrs: HashSet<usize> = HashSet::new();
    while !executed_instrs.contains(&m.pc) {
        executed_instrs.insert(m.pc);
        m.step(&instrs);
        if m.has_halted {
            return Ok(m.acc);
        }
    }
    Err(m.acc)
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

    #[test]
    fn test_part_two_test_input() {
        let instrs: Vec<(String, i32)> = TEST_INPUT
            .lines()
            .map(parse_line)
            .collect::<Result<Vec<(String, i32)>, String>>()
            .expect("failed to parse test string");
        assert_eq!(part2(instrs), 8);
    }
}
