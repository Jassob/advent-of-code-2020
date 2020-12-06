use std::{collections::HashMap, str::FromStr};

use utils;
use utils::strings;

fn main() -> Result<(), String> {
    let (part, content) = utils::parse_args()?;
    let groups = strings::split_on_empty_lines(content.as_str())
        .iter()
        .map(|strs| strings::join_lines(strs).parse().unwrap())
        .collect();
    utils::run(part1, part2, part, groups);

    Ok(())
}

fn part1(groups: Vec<Group>) -> u32 {
    groups.iter().fold(0, |acc, g| acc + g.answers.len() as u32)
}

fn part2(groups: Vec<Group>) -> u32 {
    groups.iter().fold(0, |acc, g| {
        acc + g
            .answers
            .values()
            .filter(|v| **v == g.total_members)
            .count() as u32
    })
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
        assert_eq!(part1(test_groups), 11);
    }

    #[test]
    fn test_part_two() {
        let test_groups = strings::split_on_empty_lines(TEST_STR)
            .iter()
            .map(|strs| strings::join_lines(strs).parse().unwrap())
            .collect();
        assert_eq!(part2(test_groups), 6);
    }
}
