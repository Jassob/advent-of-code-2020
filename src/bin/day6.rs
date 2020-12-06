use std::{collections::HashMap, fs};

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        Err("Usage: day6 --part <1|2> <input>")?
    }
    let part = &args[2];
    let input_file = &args[3];
    let content = fs::read_to_string(input_file)
        .or_else(|err| Err(format!("failed to read input: {}", err)))?;
    let groups = split_groups(content.as_str())
        .iter()
        .map(|s| Group::parse(s.as_str()))
        .collect();
    println!(
        "Result: {}",
        if part == "1" {
            part1(groups).ok_or_else(|| "missing")?
        } else {
            part2(groups).ok_or_else(|| "missing")?
        }
    );

    Ok(())
}

fn split_groups(input: &str) -> Vec<String> {
    input.lines().fold(vec![], |mut acc, l| {
        if l == "" {
            acc.push("".to_string());
        } else {
            if acc.last_mut().is_none() {
                acc.push(l.to_string());
            } else {
                acc.last_mut().map(|s| {
                    s.push_str("\n");
                    s.push_str(l)
                });
            }
        }
        acc
    })
}

fn part1(groups: Vec<Group>) -> Option<u32> {
    Some(
        groups
            .iter()
            .fold(0, |acc, g| acc + g.no_questions_part_one()),
    )
}

fn part2(groups: Vec<Group>) -> Option<u32> {
    Some(
        groups
            .iter()
            .fold(0, |acc, g| acc + g.no_questions_part_two()),
    )
}

#[derive(Debug, Clone)]
struct Group {
    answers: HashMap<char, u32>,
    total_members: u32,
}

impl Group {
    fn parse(input: &str) -> Group {
        Group {
            answers: input.chars().filter(|c| !char::is_whitespace(*c)).fold(
                HashMap::new(),
                |mut acc, c| {
                    let counter = acc.entry(c).or_insert(0);
                    *counter += 1;
                    acc
                },
            ),
            total_members: input.lines().filter(|s| *s != "").count() as u32,
        }
    }

    fn no_questions_part_one(&self) -> u32 {
        self.answers.len() as u32
    }

    fn no_questions_part_two(&self) -> u32 {
        self.answers
            .values()
            .filter(|v| **v == self.total_members)
            .count() as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_STR: &str = "abc

a
b
c

ab
ac

a
a
a
a

b";

    #[test]
    fn test_part_one() {
        let test_groups = split_groups(TEST_STR)
            .iter()
            .map(|s| Group::parse(s.as_str()))
            .collect();
        assert_eq!(part1(test_groups), Some(11));
    }

    #[test]
    fn test_part_two() {
        let test_groups = split_groups(TEST_STR)
            .iter()
            .map(|s| Group::parse(s.as_str()))
            .collect();
        assert_eq!(part2(test_groups), Some(6));
    }
}
