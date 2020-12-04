use std::char;
use std::fs;

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        Err("Usage: day4 --part <1|2> <input>")?
    }
    let part = &args[2];
    let input_file = &args[3];
    let passports = fs::read_to_string(input_file)
        .or_else(|err| Err(format!("failed to read input: {}", err)))
        .map(split_on_eop)
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

/// split a string on end-of-password newlines
fn split_on_eop(s: String) -> Vec<String> {
    let mut groups = vec![];
    let mut current_pass = String::new();
    for l in s.lines() {
        if l == "" {
            groups.push(current_pass);
            current_pass = String::new();
        } else {
            current_pass.push(' ');
            current_pass.push_str(l);
        }
    }
    if current_pass != "" {
        groups.push(current_pass);
    }
    groups
}

fn part1(passports: Vec<Passport>) -> u64 {
    fn fixer(p: &Passport) -> Passport {
        let mut p = p.clone();
        p.cid = Some("".to_string()); // ensure p has a cid
        p
    }
    passports
        .iter()
        .map(|p| p.is_valid(&fixer))
        .filter(|b| *b)
        .count() as u64
}

fn part2(_passports: Vec<Passport>) -> u64 {
    unimplemented!()
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
    fn new() -> Passport {
        Passport {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
        }
    }
    fn parse(input: &str) -> Passport {
        let mut p = Passport::new();
        let kvs: Vec<Vec<&str>> = input
            .split(char::is_whitespace)
            .map(|s| s.split(':').collect())
            .collect();
        for kv in kvs.iter() {
            if kv.len() != 2 {
                continue;
            }
            match kv[0] {
                "byr" => p.byr = Some(kv[1].to_string()),
                "iyr" => p.iyr = Some(kv[1].to_string()),
                "eyr" => p.eyr = Some(kv[1].to_string()),
                "hgt" => p.hgt = Some(kv[1].to_string()),
                "hcl" => p.hcl = Some(kv[1].to_string()),
                "ecl" => p.ecl = Some(kv[1].to_string()),
                "pid" => p.pid = Some(kv[1].to_string()),
                "cid" => p.cid = Some(kv[1].to_string()),
                _ => println!("ignored {:?}", kv),
            }
        }
        p
    }

    fn is_valid(&self, fixer: &dyn Fn(&Passport) -> Passport) -> bool {
        let p = fixer(self);
        p.byr.is_some()
            && p.iyr.is_some()
            && p.eyr.is_some()
            && p.hgt.is_some()
            && p.hcl.is_some()
            && p.ecl.is_some()
            && p.pid.is_some()
            && p.cid.is_some()
    }
}
