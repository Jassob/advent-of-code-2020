use std::collections::{HashMap, HashSet};

fn main() -> Result<(), String> {
    let (part, content) = utils::parse_args()?;
    utils::run(part1, part2, part, &content);
    Ok(())
}

fn part1(input: &str) -> Result<u32, String> {
    let start_numbers: Vec<u32> = input
        .split(',')
        .map(|n| {
            n.parse()
                .map_err(|e| format!("failed to parse {} as a number: {}", n, e))
        })
        .collect::<Result<Vec<u32>, String>>()?;
    Ok(play(start_numbers, 2020))
}

fn play(start_numbers: Vec<u32>, to: usize) -> u32 {
    let mut turns: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut times: HashMap<u32, u32> = HashMap::new();
    let mut last_number = 0;
    for turn in 1..to + 1 {
        let number: u32;
        if turn - 1 < start_numbers.len() {
            number = start_numbers[turn - 1];
        } else if let Some(0) = times.get(&last_number) {
            number = 0;
        } else if let Some(last_turns) = turns.get(&last_number) {
            let turns: Vec<&u32> = last_turns.iter().rev().collect();
            number = turns[0] - turns[1];
        } else {
            panic!("");
        }
        println!("turn: {}, number: {}", turn, number);
        turns
            .entry(number)
            .and_modify(|ts| ts.push(turn as u32))
            .or_insert(vec![turn as u32]);
        times.entry(number).and_modify(|t| *t += 1).or_insert(0);
        last_number = number;
    }
    last_number
}

fn part2(input: &str) -> Result<u32, String> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_play() {
        assert_eq!(play(vec![0, 3, 6], 10), 0);
    }

    #[test]
    fn test_part_one_1() {
        assert_eq!(part1("1,3,2"), Ok(1));
    }

    #[test]
    fn test_part_one_2() {
        assert_eq!(part1("2,1,3"), Ok(10));
    }

    #[test]
    fn test_part_one_3() {
        assert_eq!(part1("1,2,3"), Ok(27));
    }
}
