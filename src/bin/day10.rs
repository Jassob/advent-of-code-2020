use std::str::FromStr;

fn main() -> Result<(), String> {
    let (part, content) = utils::parse_args()?;
    let input = parse_input(&content)?;
    utils::run(part1, part2, part, input);
    Ok(())
}

fn parse_input(s: &str) -> Result<Vec<u32>, String> {
    s.lines()
        .map(|l| {
            l.parse()
                .map_err(|e| format!("failed to parse {} as a number: {}", l, e))
        })
        .collect()
}

fn part1(input: Vec<u32>) -> Result<u64, String> {
    if let Ok((ones, _, threes)) = find_differences(input) {
        return Ok(ones * threes);
    };
    Err("failed to find a chain of adapters".to_string())
}

fn find_differences(mut input: Vec<u32>) -> Result<(u64, u64, u64), String> {
    input.sort();
    let (mut diff_one, mut diff_two, mut diff_three, mut last) = (0, 0, 0, 0);
    for n in input.iter() {
        match *n - last {
            1 => diff_one += 1,
            2 => diff_two += 1,
            3 => diff_three += 1,
            d => println!("diff is not one of 1,2 or 3: {}", d),
        };
        last = *n;
    }
    Ok((diff_one, diff_two, diff_three + 1))
}

fn part2(input: Vec<u32>) -> Result<u64, String> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_STR_SHORT: &str = "16
10
15
5
1
11
7
19
6
12
4";

    #[test]
    fn test_part_one_short() {
        let input = parse_input(TEST_STR_SHORT).expect("should not fail");
        assert_eq!(find_differences(input), Ok((7, 0, 5)))
    }

    const TEST_STR_LONG: &str = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";

    #[test]
    fn test_part_one_long() {
        let input = parse_input(TEST_STR_LONG).expect("should not fail");
        assert_eq!(find_differences(input), Ok((22, 0, 10)));
    }
}
