use std::{clone::Clone, fmt::Display, fs, str::FromStr};

pub mod strings;

#[derive(Debug, PartialEq)]
pub enum Part {
    One,
    Two,
    Both,
}

impl FromStr for Part {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Part::One),
            "2" => Ok(Part::Two),
            "both" => Ok(Part::Both),
            _ => Err(format!("expected either '1', '2' or 'both', found {}", s)),
        }
    }
}

/// Parse arguments for a advent of code problem.
///
/// Returns the part to be run and the content of the INPUT_FILE.
///
/// Assumptions:
/// Usage: day --part <1|2|both> INPUT_FILE
pub fn parse_args() -> Result<(Part, String), String> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        Err(format!("Usage: {} --part <1|2|both> <input>", args[0]))?
    }
    let part: Part = args[2]
        .parse::<Part>()
        .or_else(|err| Err(format!("failed to read part: {}", err)))?;
    let input_file = &args[3];
    let content = fs::read_to_string(input_file)
        .or_else(|err| Err(format!("failed to read input: {}", err)))?;
    Ok((part, content))
}

/// Run the part functions, depending on what part is provided.
pub fn run<In: Clone, Out: Display>(
    part1: fn(In) -> Out,
    part2: fn(In) -> Out,
    part: Part,
    input: In,
) {
    if part == Part::One {
        println!("Part 1: {}", part1(input));
    } else if part == Part::Two {
        println!("Part 2: {}", part2(input));
    } else {
        println!("Part 1: {}", part1(input.clone()));
        println!("Part 2: {}", part2(input));
    }
}
