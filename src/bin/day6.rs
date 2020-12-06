use std::{collections::HashMap, fs, str::FromStr};

use utils::strings;

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        Err("Usage: day6 --part <1|2> <input>")?
    }
    let part = &args[2];
    let input_file = &args[3];
    let content = fs::read_to_string(input_file)
        .or_else(|err| Err(format!("failed to read input: {}", err)))?;
    let groups = strings::split_on_empty_lines(content.as_str())
        .iter()
        .map(|strs| strings::join_lines(strs).parse().unwrap())
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

fn part1(groups: Vec<Group>) -> Option<u32> {
    Some(groups.iter().fold(0, |acc, g| acc + g.answers.len() as u32))
}

fn part2(groups: Vec<Group>) -> Option<u32> {
    Some(groups.iter().fold(0, |acc, g| {
        acc + g
            .answers
            .values()
            .filter(|v| **v == g.total_members)
            .count() as u32
    }))
}

#[derive(Debug, Clone)]
struct Group {
    answers: HashMap<char, u32>,
    total_members: u32,
}

impl FromStr for Group {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Group {
            answers: s.chars().filter(|c| !char::is_whitespace(*c)).fold(
                HashMap::new(),
                |mut acc, c| {
                    let counter = acc.entry(c).or_insert(0);
                    *counter += 1;
                    acc
                },
            ),
            total_members: s.lines().count() as u32,
        })
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
        let test_groups = strings::split_on_empty_lines(TEST_STR)
            .iter()
            .map(|strs| strings::join_lines(strs).parse().unwrap())
            .collect();
        println!("{:?}", test_groups);
        assert_eq!(part1(test_groups), Some(11));
    }

    #[test]
    fn test_part_two() {
        let test_groups = strings::split_on_empty_lines(TEST_STR)
            .iter()
            .map(|strs| strings::join_lines(strs).parse().unwrap())
            .collect();
        assert_eq!(part2(test_groups), Some(6));
    }
}
