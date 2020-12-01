use std::fs;

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        Err("Usage: day1 --part <1|2> <input>")?
    }
    let part = &args[2];
    let input_file = &args[3];
    let content = fs::read_to_string(input_file)
        .or_else(|err| Err(format!("failed to read input: {}", err)))
        .and_then(parse_input)?;
    match if part == "1" {
        part1(content)
    } else {
        part2(content)
    } {
        Some(val) => println!("Result: {}", val),
        None => Err("could not find any bad input")?,
    }

    Ok(())
}

fn parse_input(input: String) -> Result<Vec<u32>, String> {
    input.lines().try_fold(vec![], |mut acc, line| {
        line.parse::<u32>()
            .and_then(|val| {
                acc.push(val);
                Ok(acc)
            })
            .or_else(|err| Err(format!("failed to parse number: {}", err)))
    })
}

fn part1(numbers: Vec<u32>) -> Option<u32> {
    for i in numbers.iter() {
        for j in numbers.iter() {
            if i + j == 2020 {
                return Some(i * j);
            }
        }
    }
    None
}

fn part2(numbers: Vec<u32>) -> Option<u32> {
    for i in numbers.iter() {
        for j in numbers.iter() {
            for k in numbers.iter() {
                if i + j + k == 2020 {
                    return Some(i * j * k);
                }
            }
        }
    }
    None
}
