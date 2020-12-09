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
    let n = first_violation(&input, 25).ok_or_else(|| "failed to solve part1".to_string())?;
    Ok(n)
}

fn part2(input: Vec<i64>) -> Result<i64, String> {
    encryption_weakness(&input, 25).ok_or_else(|| "failed to solve part 2".to_string())
}

fn first_violation(input: &Vec<i64>, preamble_len: usize) -> Option<i64> {
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

fn encryption_weakness(input: &Vec<i64>, preamble_len: usize) -> Option<i64> {
    let violation = first_violation(input, preamble_len)?;
    let mut start = 0;
    let mut sum = 0;
    for (i, n) in input.iter().enumerate() {
        sum += n;
        while sum > violation {
            sum -= input[start];
            start += 1;
        }
        if sum == violation {
            let range: Vec<i64> = input[start..i].iter().map(|n| *n).collect();
            return Some(*range.iter().min().unwrap() + *range.iter().max().unwrap());
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
        assert_eq!(first_violation(&nums, 5), Some(127));
    }

    #[test]
    fn test_part_two() {
        let nums = parse_lines(TEST_INPUT).unwrap();
        assert_eq!(encryption_weakness(&nums, 5), Some(62));
    }
}
