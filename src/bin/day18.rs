use std::str::FromStr;

fn main() -> Result<(), String> {
    let (part, content) = utils::parse_args()?;
    let input = content.lines().map(|s| s.to_string()).collect();
    utils::run(part1, part2, part, input);
    Ok(())
}

fn eval(line: &str) -> u32 {
    let mut val = 0;
    let mut words = line.split(' ');
    loop {}
    unimplemented!()
}

fn part1(input: Vec<String>) -> u32 {
    unimplemented!()
}

fn part2(_input: Vec<String>) -> u32 {
    unimplemented!()
}

enum Expr {
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Lit(i32),
}

impl FromStr for Expr {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval() {
        assert_eq!(eval("2 * 3 + (4 * 5)"), 26);
    }
}
