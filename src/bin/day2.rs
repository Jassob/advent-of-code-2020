use std::convert::TryInto;
use std::fs;
use std::str::Chars;

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        Err("Usage: day1 --part <1|2> <input>")?
    }
    let part = &args[2];
    let input_file = &args[3];
    let passwords = fs::read_to_string(input_file)
        .or_else(|err| Err(format!("failed to read input: {}", err)))
        .and_then(parse_passwords)?;
    if part == "1" {
        println!("Result: {}", part1(passwords));
    } else {
        println!("Result: {}", part2(passwords));
    }

    Ok(())
}

fn parse_passwords(input: String) -> Result<Vec<Password>, String> {
    input
        .lines()
        .map(|l| Password::parse(l.to_string()))
        .collect()
}

fn part1(numbers: Vec<Password>) -> u32 {
    numbers
        .iter()
        .map(|p| p.is_valid())
        .filter(|o| *o)
        .count()
        .try_into()
        .unwrap()
}

fn part2(numbers: Vec<Password>) -> u32 {
    numbers
        .iter()
        .map(|p| p.is_valid_new_policy())
        .filter(|b| *b)
        .count()
        .try_into()
        .unwrap()
}

#[derive(Debug)]
struct Password {
    min: u32,
    max: u32,
    character: char,
    password: String,
}

impl Password {
    fn is_valid(&self) -> bool {
        let count: u32 = self
            .password
            .chars()
            .filter(|c| *c == self.character)
            .count()
            .try_into()
            .unwrap();
        return count >= self.min && count <= self.max;
    }

    fn is_valid_new_policy(&self) -> bool {
        let first_index: usize = self.min.try_into().unwrap();
        let second_index: usize = self.max.try_into().unwrap();
        let first = self.password.chars().nth(first_index - 1).unwrap() == self.character;
        let second = self.password.chars().nth(second_index - 1).unwrap() == self.character;
        (!first && second) || (first && !second)
    }

    /// TODO: Make use of real parser
    fn parse(line: String) -> Result<Password, String> {
        let mut words = line.split(' ');
        let bounds_str = words.next().ok_or_else(|| "bound group missing")?;
        let mut bounds = bounds_str.chars();
        let min =
            consume_digit(&mut bounds).map_err(|err| format!("failed to parse min: {}", err))?;
        let max =
            consume_digit(&mut bounds).map_err(|err| format!("failed to parse max: {}", err))?;

        let character = words
            .next()
            .ok_or_else(|| "character group missing")?
            .chars()
            .next()
            .ok_or_else(|| "unexpected end of string".to_string())?;

        let password: String = words
            .next()
            .ok_or_else(|| "last group missing".to_string())?
            .to_string();
        Ok(Password {
            max: max,
            min: min,
            character: character,
            password: password,
        })
    }
}

fn consume_digit(chars: &mut Chars) -> Result<u32, String> {
    let num: u32 = chars
        .take_while(|c| c.is_digit(10))
        .collect::<String>()
        .parse()
        .or_else(|err| Err(format!("failed to parse digit: {}", err)))?;
    Ok(num)
}
