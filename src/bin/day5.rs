use std::char;
use std::fs;

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        Err("Usage: day5 --part <1|2> <input>")?
    }
    let part = &args[2];
    let input_file = &args[3];
    let passports = fs::read_to_string(input_file)
        .or_else(|err| Err(format!("failed to read input: {}", err)))?
        .lines()
        .map(|l| BoardingPass::parse(l).expect("no invalid boarding passes"))
        .collect();

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

fn part1(boardingpasses: Vec<BoardingPass>) -> u64 {
    boardingpasses
        .iter()
        .map(|b| b.seat().id())
        .max()
        .unwrap_or(0)
}

fn part2(_boardingpasses: Vec<BoardingPass>) -> u64 {
    unimplemented!()
}

#[derive(Debug, Clone)]
struct BoardingPass {
    row: Vec<Direction>,
    column: Vec<Direction>,
}

impl BoardingPass {
    fn parse(input: &str) -> Option<BoardingPass> {
        if input.len() != 10 {
            None
        } else {
            let (rows, columns) = input.split_at(7);
            Some(BoardingPass {
                row: rows
                    .chars()
                    .map(|c| Direction::from_char(c, 'F', 'B').expect("should not fail"))
                    .collect(),
                column: columns
                    .chars()
                    .map(|c| Direction::from_char(c, 'L', 'R').expect("should not fail"))
                    .collect(),
            })
        }
    }

    fn seat(&self) -> Seat {
        Seat {
            row: bsp(&self.row, Seat::MAX_ROW as u64).unwrap_or(0),
            column: bsp(&self.column, Seat::MAX_COLUMN as u64).unwrap_or(0),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Direction {
    Higher,
    Lower,
}

impl Direction {
    fn from_char(c: char, low: char, high: char) -> Option<Direction> {
        if c == high {
            Some(Direction::Higher)
        } else if c == low {
            Some(Direction::Lower)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Seat {
    row: u64,
    column: u64,
}

impl Seat {
    const MAX_ROW: u8 = 127;
    const MAX_COLUMN: u8 = 7;

    fn id(&self) -> u64 {
        self.row * 8 + self.column
    }
}

fn bsp(dirs: &Vec<Direction>, max: u64) -> Option<u64> {
    let mut slots: Vec<u64> = (0..max + 1).collect();
    for d in dirs.iter() {
        let length = slots.len();
        if *d == Direction::Higher {
            slots = slots.iter().skip(length / 2).map(|s| *s).collect();
        } else {
            slots = slots.iter().take(length / 2).map(|s| *s).collect();
        }
    }
    if slots.len() != 1 {
        None
    } else {
        Some(slots[0])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bsp_1() {
        let row_dirs: Vec<Direction> = "FBFBBFF"
            .chars()
            .map(|c| Direction::from_char(c, 'F', 'B').expect("no fail"))
            .collect();
        let column_dirs: Vec<Direction> = "RLR"
            .chars()
            .map(|c| Direction::from_char(c, 'L', 'R').expect("no fail"))
            .collect();
        assert_eq!(bsp(&row_dirs, Seat::MAX_ROW as u64), Some(44));
        assert_eq!(bsp(&column_dirs, Seat::MAX_COLUMN as u64), Some(5));
    }

    #[test]
    fn part_1_tests() {
        struct Test {
            line: &'static str,
            expected_seat: Seat,
            expected_seat_id: u64,
        }

        for t in vec![
            Test {
                line: "BFFFBBFRRR",
                expected_seat: Seat { row: 70, column: 7 },
                expected_seat_id: 567,
            },
            Test {
                line: "FFFBBBFRRR",
                expected_seat: Seat { row: 14, column: 7 },
                expected_seat_id: 119,
            },
            Test {
                line: "BBFFBBFRLL",
                expected_seat: Seat {
                    row: 102,
                    column: 4,
                },
                expected_seat_id: 820,
            },
        ] {
            let bp = BoardingPass::parse(t.line);
            let seat = bp.expect("no fail").seat();
            assert_eq!(seat, t.expected_seat);
            assert_eq!(seat.id(), t.expected_seat_id);
        }
    }
}
