use std::str::{Chars, FromStr};

use utils;

fn main() -> Result<(), String> {
    let (part, content) = utils::parse_args()?;
    let passwords: Vec<Password> = content
        .lines()
        .map(|l| l.parse())
        .collect::<Result<Vec<Password>, String>>()?;
    utils::run(part1, part2, part, passwords);

    Ok(())
}

fn part1(numbers: Vec<Password>) -> u32 {
    numbers.iter().map(|p| p.is_valid()).filter(|o| *o).count() as u32
}

fn part2(numbers: Vec<Password>) -> u32 {
    numbers
        .iter()
        .map(|p| p.is_valid_new_policy())
        .filter(|b| *b)
        .count() as u32
}

#[derive(Debug, Clone)]
struct Password {
    min: u32,
    max: u32,
    character: char,
    password: String,
}

impl Password {
    fn is_valid(&self) -> bool {
        let count = self
            .password
            .chars()
            .filter(|c| *c == self.character)
            .count() as u32;
        return count >= self.min && count <= self.max;
    }

    fn is_valid_new_policy(&self) -> bool {
        let first = self.password.chars().nth(self.min as usize - 1) == Some(self.character);
        let second = self.password.chars().nth(self.max as usize - 1) == Some(self.character);
        (!first && second) || (first && !second)
    }
}

impl FromStr for Password {
    type Err = String;
    fn from_str(line: &str) -> Result<Self, Self::Err> {
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
            min,
            max,
            character,
            password,
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
