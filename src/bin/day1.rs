fn main() -> Result<(), String> {
    let (part, content) = utils::parse_args()?;
    let input = content
        .lines()
        .map(|l| {
            l.parse::<u32>()
                .map_err(|e| format!("failed to parse number: {}", e))
        })
        .collect::<Result<Vec<u32>, String>>()?;
    utils::run(part1, part2, part, input);

    Ok(())
}

fn part1(numbers: Vec<u32>) -> u32 {
    for i in numbers.iter() {
        for j in numbers.iter() {
            if i + j == 2020 {
                return i * j;
            }
        }
    }
    panic!("found no such number pair");
}

fn part2(numbers: Vec<u32>) -> u32 {
    for i in numbers.iter() {
        for j in numbers.iter() {
            for k in numbers.iter() {
                if i + j + k == 2020 {
                    return i * j * k;
                }
            }
        }
    }
    panic!("found no such number triple");
}
