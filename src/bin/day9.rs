fn main() -> Result<(), String> {
    let (part, content) = utils::parse_args()?;
    let input = parse_lines(&content)?;
    utils::run(part1, part2, part, input);
    Ok(())
}

fn parse_lines(s: &str) -> Result<Vec<i64>, String> {
    let numbers: Vec<i64> = s
        .lines()
        .map(|l| {
            l.parse::<i64>()
                .map_err(|e| format!("failed to parse {} as a number: {}", l, e))
        })
        .collect::<Result<Vec<i64>, String>>()?;
    Ok(numbers)
}

fn part1(input: Vec<i64>) -> Result<i64, String> {
    first_violation(input, 25).ok_or_else(|| "failed to solve part1".to_string())
}

fn part2(input: Vec<i64>) -> Result<i64, String> {
    unimplemented!()
}

fn first_violation(input: Vec<i64>, preamble_len: usize) -> Option<i64> {
    let mut window: Vec<i64> = input.iter().take(preamble_len).map(|n| *n).collect();
    for n in input.iter().skip(preamble_len) {
        let last_window: Vec<&i64> = window.iter().rev().take(preamble_len).collect();
        if last_window
            .iter()
            .any(|w1| last_window.iter().filter(|w2| **w1 + **w2 == *n).count() > 0)
        {
            window.push(*n);
        } else {
            return Some(*n);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

    #[test]
    fn test_part_one() {
        let nums = parse_lines(TEST_INPUT).unwrap();
        assert_eq!(first_violation(nums, 5), Some(127));
    }
}
