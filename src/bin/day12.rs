use std::str::FromStr;

fn main() -> Result<(), String> {
    let (part, content) = utils::parse_args()?;
    let input = parse_input(&content)?;
    utils::run(part1, part2, part, input);
    Ok(())
}

fn parse_input(s: &str) -> Result<Vec<Direction>, String> {
    s.lines().map(|l| l.parse()).collect()
}

fn part1(input: Vec<Direction>) -> Result<i32, String> {
    let mut s = Ship::new();
    s.follow(&input);
    Ok(s.north.abs() + s.east.abs())
}

fn part2(_input: Vec<Direction>) -> Result<i32, String> {
    unimplemented!()
}

#[derive(Clone, Debug)]
enum Direction {
    Forward(usize),
    North(usize),
    East(usize),
    South(usize),
    West(usize),
    Right(i32),
    Left(i32),
}

impl FromStr for Direction {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 2 {
            return Err(format!(
                "invalid length of line, expected at least 2 characters but got: {}",
                s
            ));
        }
        let (d, num) = s.split_at(1);
        let parsed_num = num
            .parse()
            .map_err(|e| format!("failed to parse {} as a number: {}", num, e))?;
        let dir = match d {
            "F" => Direction::Forward(parsed_num as usize),
            "N" => Direction::North(parsed_num as usize),
            "E" => Direction::East(parsed_num as usize),
            "S" => Direction::South(parsed_num as usize),
            "W" => Direction::West(parsed_num as usize),
            "R" => Direction::Right(parsed_num),
            "L" => Direction::Left(parsed_num),
            _ => Err(format!(
                "invalid direction, expected one of F, N, E, S, W, R or L but found: {}",
                d
            ))?,
        };
        Ok(dir)
    }
}

#[derive(Clone, Debug)]
struct Ship {
    north: i32,
    east: i32,
    heading: i32,
}

impl Ship {
    fn new() -> Ship {
        Ship {
            north: 0,
            east: 0,
            heading: 0,
        }
    }

    fn follow(&mut self, directions: &Vec<Direction>) {
        directions.iter().for_each(|d| self.follow_direction(d))
    }

    fn follow_direction(&mut self, direction: &Direction) {
        match *direction {
            Direction::North(n) => self.north += n as i32,
            Direction::East(n) => self.east += n as i32,
            Direction::South(n) => self.north -= n as i32,
            Direction::West(n) => self.east -= n as i32,
            Direction::Right(deg) => self.heading = (self.heading + deg) % 360,
            Direction::Left(deg) => self.heading = (self.heading - deg + 360) % 360,
            Direction::Forward(n) => match self.heading {
                0 => self.east += n as i32,
                90 => self.north -= n as i32,
                180 => self.east -= n as i32,
                270 => self.north += n as i32,
                _ => println!("bad direction: {}", self.heading),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_STR: &str = "F10
N3
F7
R90
F11";

    #[test]
    fn test_part_one() {
        let directions = parse_input(TEST_STR).expect("no fail");
        assert_eq!(part1(directions), Ok(25));
    }
}
