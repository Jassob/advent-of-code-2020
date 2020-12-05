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
        .map(|l| BoardingPass::parse(l))
        .collect();

    println!(
        "Result: {}",
        if part == "1" {
            part1(passports).ok_or_else(|| "missing")?
        } else {
            part2(passports).ok_or_else(|| "missing")?
        }
    );

    Ok(())
}

fn part1(boardingpasses: Vec<BoardingPass>) -> Option<u64> {
    boardingpasses.iter().map(|b| b.seat_id()).max()
}

fn part2(boardingpasses: Vec<BoardingPass>) -> Option<u64> {
    let mut seats = boardingpasses
        .iter()
        .map(|bp| bp.seat_id())
        .collect::<Vec<u64>>();
    seats.sort();
    // pair up all seat ids with its successor id and see where the gap is
    seats
        .iter()
        .zip(seats.iter().skip(1))
        .find(|(id1, id2)| *id2 - *id1 > 1)
        // return missing id
        .and_then(|(id1, id2)| (*id1..*id2).nth(1))
}

#[derive(Debug, Clone, PartialEq)]
struct BoardingPass {
    row: u8,
    column: u8,
}

impl BoardingPass {
    fn parse(input: &str) -> BoardingPass {
        fn parse_as_u8(s: &str, from_char: &dyn Fn(char) -> u8) -> u8 {
            s.chars()
                .map(from_char)
                .rev()
                .enumerate()
                .map(|(i, d)| d * 2u8.pow(i as u32))
                .sum()
        }
        let (rows, columns) = input.split_at(7);
        BoardingPass {
            row: parse_as_u8(rows, &|c| if c == 'B' { 1 } else { 0 }),
            column: parse_as_u8(columns, &|c| if c == 'R' { 1 } else { 0 }),
        }
    }

    fn seat_id(&self) -> u64 {
        self.row as u64 * 8 + self.column as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_tests() {
        struct Test {
            line: &'static str,
            expected_boardingpass: BoardingPass,
            expected_seat_id: u64,
        }

        for t in vec![
            Test {
                line: "BFFFBBFRRR",
                expected_boardingpass: BoardingPass { row: 70, column: 7 },
                expected_seat_id: 567,
            },
            Test {
                line: "FFFBBBFRRR",
                expected_boardingpass: BoardingPass { row: 14, column: 7 },
                expected_seat_id: 119,
            },
            Test {
                line: "BBFFBBFRLL",
                expected_boardingpass: BoardingPass {
                    row: 102,
                    column: 4,
                },
                expected_seat_id: 820,
            },
        ] {
            let bp = BoardingPass::parse(t.line);
            assert_eq!(bp, t.expected_boardingpass);
            let seat_id = bp.seat_id();
            assert_eq!(seat_id, t.expected_seat_id);
        }
    }
}
