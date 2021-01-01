use core::fmt;
use std::{collections::HashMap, str::FromStr};

fn main() -> Result<(), String> {
    let (part, content) = utils::parse_args()?;
    let input = content.parse()?;
    utils::run(part1, part2, part, input);
    Ok(())
}

fn part1(mut input: Room) -> Result<i64, String> {
    loop {
        input.step(utils::Part::One);
        if input.stable {
            break;
        }
    }
    Ok(input
        .positions
        .values()
        .filter(|v| v.map_or_else(|| false, |s| s == Seat::Occupied))
        .count() as i64)
}

fn part2(mut input: Room) -> Result<i64, String> {
    loop {
        input.step(utils::Part::Two);
        if input.stable {
            break;
        }
    }
    Ok(input
        .positions
        .values()
        .filter(|v| v.map_or_else(|| false, |s| s == Seat::Occupied))
        .count() as i64)
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Seat {
    Empty,
    Occupied,
}

#[derive(Clone, Debug, PartialEq)]
struct Room {
    positions: HashMap<(i32, i32), Option<Seat>>,
    max_row: usize,
    max_column: usize,
    stable: bool,
}

impl Room {
    fn step(&mut self, part: utils::Part) {
        let mut changes = 0;
        let seats_to_check: Vec<(i32, i32)> = self
            .positions
            .iter()
            .filter(|(_, v)| v.is_some())
            .map(|(k, _)| *k)
            .collect();
        let mut new_positions = self.positions.clone();
        for seat in seats_to_check {
            let new_value = match part {
                utils::Part::One => self.check_seat(seat),
                utils::Part::Two => self.check_seat_part2(seat),
                _ => unimplemented!(),
            };
            if self.positions[&seat] != new_value {
                new_positions.insert(seat, new_value);
                changes += 1;
            }
        }
        self.positions = new_positions;
        self.stable = changes == 0;
    }

    fn check_seat(&self, s: (i32, i32)) -> Option<Seat> {
        let neighbors = self.get_adjacent_cells(s);
        match self.positions[&s] {
            None => None,
            Some(Seat::Occupied) => {
                if neighbors
                    .iter()
                    .filter(|n| **n == Some(Seat::Occupied))
                    .count()
                    >= 4
                {
                    Some(Seat::Empty)
                } else {
                    Some(Seat::Occupied)
                }
            }
            Some(Seat::Empty) => {
                if neighbors.iter().all(|n| *n != Some(Seat::Occupied)) {
                    Some(Seat::Occupied)
                } else {
                    Some(Seat::Empty)
                }
            }
        }
    }

    fn check_seat_part2(&self, s: (i32, i32)) -> Option<Seat> {
        let neighbors = self.get_neighbors(s);
        match self.positions[&s] {
            None => None,
            Some(Seat::Occupied) => {
                if neighbors
                    .iter()
                    .cloned()
                    .filter(|n| *n == Seat::Occupied)
                    .count()
                    >= 5
                {
                    Some(Seat::Empty)
                } else {
                    Some(Seat::Occupied)
                }
            }
            Some(Seat::Empty) => {
                if neighbors.iter().all(|n| *n != Seat::Occupied) {
                    Some(Seat::Occupied)
                } else {
                    Some(Seat::Empty)
                }
            }
        }
    }

    fn get_adjacent_cells(&self, (row, col): (i32, i32)) -> Vec<Option<Seat>> {
        let mut neighbors = vec![];
        for r in (-1..2).map(|i| row + i) {
            for c in (-1..2).map(|i| col + i) {
                if (r, c) == (row, col) {
                    continue;
                }
                if let Some(p) = self.positions.get(&(r, c)) {
                    neighbors.push(*p);
                }
            }
        }
        neighbors
    }

    fn get_neighbors(&self, (row, col): (i32, i32)) -> Vec<Seat> {
        enum Direction {
            N,
            NE,
            E,
            SE,
            S,
            SW,
            W,
            NW,
        }

        fn get_neighbor_in_direction(
            (row, col): (i32, i32),
            direction: Direction,
            room: &Room,
        ) -> Option<Seat> {
            let pos = match direction {
                Direction::N => (row - 1, col),
                Direction::NE => (row - 1, col + 1),
                Direction::E => (row, col + 1),
                Direction::SE => (row + 1, col + 1),
                Direction::S => (row + 1, col),
                Direction::SW => (row + 1, col - 1),
                Direction::W => (row, col - 1),
                Direction::NW => (row - 1, col - 1),
            };
            match room.positions.get(&pos) {
                None => None,
                Some(None) => get_neighbor_in_direction(pos, direction, room),
                Some(Some(s)) => Some(*s),
            }
        }

        vec![
            Direction::N,
            Direction::NE,
            Direction::E,
            Direction::SE,
            Direction::S,
            Direction::SW,
            Direction::W,
            Direction::NW,
        ]
        .into_iter()
        .filter_map(|d| get_neighbor_in_direction((row, col), d, self))
        .collect::<Vec<Seat>>()
    }
}

impl FromStr for Room {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut room = HashMap::new();
        let mut max_row = 0;
        let mut max_column = 0;
        for (row, l) in s.lines().enumerate() {
            max_row = row;
            l.chars().enumerate().for_each(|(col, c)| {
                match c {
                    '#' => room.insert((row as i32, col as i32), Some(Seat::Occupied)),
                    'L' => room.insert((row as i32, col as i32), Some(Seat::Empty)),
                    '.' => room.insert((row as i32, col as i32), None),
                    _ => room.insert((row as i32, col as i32), None),
                };
                max_column = col;
            })
        }
        Ok(Room {
            positions: room,
            max_row,
            max_column,
            stable: false,
        })
    }
}

impl fmt::Display for Room {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), std::fmt::Error> {
        for row in 0..self.max_row + 1 {
            for column in 0..self.max_column + 1 {
                match self.positions.get(&(row as i32, column as i32)).unwrap() {
                    Some(Seat::Empty) => write!(f, "{}", 'L')?,
                    Some(Seat::Occupied) => write!(f, "{}", '#')?,
                    None => write!(f, "{}", '.')?,
                };
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_STR: &str = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    #[test]
    fn test_step() {
        let mut r: Room = "#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##"
            .parse()
            .unwrap();
        let expected: Room = "#.LL.L#.##
#LLLLLL.L#
L.L.L..L..
#LLL.LL.L#
#.LL.LL.LL
#.LLLL#.##
..L.L.....
#LLLLLLLL#
#.LLLLLL.L
#.#LLLL.##"
            .parse()
            .unwrap();
        r.step(utils::Part::One);
        assert_eq!(r, expected);
    }

    #[test]
    fn test_part_one() {
        let r = TEST_STR.parse().unwrap();
        assert_eq!(part1(r), Ok(37))
    }

    #[test]
    fn test_step_part_two() {
        let mut r: Room = "#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##"
            .parse()
            .unwrap();
        let expected: Room = "#.LL.LL.L#
#LLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLL#
#.LLLLLL.L
#.LLLLL.L#"
            .parse()
            .unwrap();
        r.step(utils::Part::Two);
        assert_eq!(r, expected);
    }

    #[test]
    fn test_part_two() {
        let r = TEST_STR.parse().unwrap();
        assert_eq!(part2(r), Ok(26))
    }
}
