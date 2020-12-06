use std::char;
use std::collections::HashMap;
use std::fs;

use utils::strings;

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        Err("Usage: day4 --part <1|2> <input>")?
    }
    let part = &args[2];
    let input_file = &args[3];
    let passports = fs::read_to_string(input_file)
        .or_else(|err| Err(format!("failed to read input: {}", err)))
        .map(|s| {
            strings::split_on_empty_lines(s)
                .iter()
                .map(strings::join_lines)
                .collect::<Vec<String>>()
        })
        .and_then(|passports| Ok(passports.iter().map(|p| Passport::parse(p)).collect()))?;

    println!(
        "Result: {}",
        if part == "1" {
            part1(passports)
        } else {
            part2(passports)
        }
    );

    Ok(())
}

fn part1(passports: Vec<Passport>) -> u64 {
    fn validater(p: &Passport) -> bool {
        p.byr.is_some()
            && p.iyr.is_some()
            && p.eyr.is_some()
            && p.hgt.is_some()
            && p.hcl.is_some()
            && p.ecl.is_some()
            && p.pid.is_some()
    }

    passports
        .iter()
        .map(|p| p.is_valid(&validater))
        .filter(|b| *b)
        .count() as u64
}

fn part2(passports: Vec<Passport>) -> u64 {
    fn valid_eye_color(s: &String) -> bool {
        ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&s.as_str())
    }
    fn within_bounds(s: Option<&String>, min: u64, max: u64) -> bool {
        s.and_then(|s| s.parse::<u64>().ok())
            .map(|y| (min..max + 1).contains(&y))
            .unwrap_or(false)
    }
    fn check_height(s: &String) -> bool {
        let (number, unit): (String, String) = s.chars().partition(|c| char::is_digit(*c, 10));
        let length: u64 = number.parse().expect("a sequence of digits");
        if unit.as_str() == "in" {
            (59..76 + 1).contains(&length)
        } else if unit.as_str() == "cm" {
            (150..193 + 1).contains(&length)
        } else {
            false
        }
    }
    fn check_hex_color(s: &String) -> bool {
        let (first, rest) = s.split_at(1);
        first == "#" && rest.len() == 6 && rest.chars().all(|s| char::is_ascii_hexdigit(&s))
    }
    fn validater(p: &Passport) -> bool {
        let valid_hgt = p.hgt.as_ref().map(&check_height).unwrap_or(false);
        let valid_hcl = p.hcl.as_ref().map(&check_hex_color).unwrap_or(false);
        let valid_ecl = p.ecl.as_ref().map(&valid_eye_color).unwrap_or(false);
        let valid_pid = p
            .pid
            .as_ref()
            .map(|s| s.len() == 9 && s.parse::<u64>().is_ok())
            .unwrap_or(false);
        within_bounds(p.byr.as_ref(), 1920, 2002)
            && within_bounds(p.iyr.as_ref(), 2010, 2020)
            && within_bounds(p.eyr.as_ref(), 2020, 2030)
            && valid_hgt
            && valid_hcl
            && valid_ecl
            && valid_pid
    }
    passports
        .iter()
        .map(|p| p.is_valid(&validater))
        .filter(|b| *b)
        .count() as u64
}

#[derive(Debug, Clone)]
struct Passport {
    byr: Option<String>, // (Birth Year)
    iyr: Option<String>, // (Issue Year)
    eyr: Option<String>, // (Expiration Year)
    hgt: Option<String>, // (Height)
    hcl: Option<String>, // (Hair Color)
    ecl: Option<String>, // (Eye Color)
    pid: Option<String>, // (Passport ID)
    cid: Option<String>, // (Country ID)
}

impl Passport {
    fn from_hashmap(dict: HashMap<String, String>) -> Passport {
        Passport {
            byr: dict.get("byr").map(|s| s.to_string()),
            iyr: dict.get("iyr").map(|s| s.to_string()),
            eyr: dict.get("eyr").map(|s| s.to_string()),
            hgt: dict.get("hgt").map(|s| s.to_string()),
            hcl: dict.get("hcl").map(|s| s.to_string()),
            ecl: dict.get("ecl").map(|s| s.to_string()),
            pid: dict.get("pid").map(|s| s.to_string()),
            cid: dict.get("cid").map(|s| s.to_string()),
        }
    }

    fn parse(input: &str) -> Passport {
        let kvs: HashMap<String, String> = input
            .split(char::is_whitespace)
            .filter(|s| *s != "")
            .map(|s| {
                (
                    s.chars().take(3).collect::<String>(),
                    s.chars().skip(4).collect::<String>(),
                )
            })
            .collect();
        Passport::from_hashmap(kvs)
    }

    fn is_valid(&self, validater: &dyn Fn(&Passport) -> bool) -> bool {
        validater(self)
    }
}
