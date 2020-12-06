use std::fs;

use utils;

fn main() -> Result<(), String> {
    let (part, content) = utils::parse_args()?;
    let seats = content.lines().map(parse_seats_id).collect();
    utils::run(part1, part2, part, seats);

    Ok(())
}

fn part1(seats: Vec<u64>) -> u64 {
    seats.iter().max().map(|v| *v).expect("found no max")
}

fn part2(seats: Vec<u64>) -> u64 {
    let mut seats = seats.clone();
    seats.sort();
    // pair up all seat ids with its successor id and see where the gap is
    seats
        .iter()
        .zip(seats.iter().skip(1))
        .find(|(id1, id2)| *id2 - *id1 > 1)
        // return missing id
        .and_then(|(id1, id2)| (*id1..*id2).nth(1))
        .expect("found no hole")
}

fn parse_seats_id(input: &str) -> u64 {
    input
        .chars()
        .map(|c| if c == 'B' || c == 'R' { 1 } else { 0 })
        .fold(0, |acc, b| (acc << 1) + b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_tests() {
        struct Test {
            line: &'static str,
            expected_seat_id: u64,
        }

        for t in vec![
            Test {
                line: "BFFFBBFRRR",
                expected_seat_id: 567,
            },
            Test {
                line: "FFFBBBFRRR",
                expected_seat_id: 119,
            },
            Test {
                line: "BBFFBBFRLL",
                expected_seat_id: 820,
            },
        ] {
            let seat_id = parse_seats_id(t.line);
            assert_eq!(seat_id, t.expected_seat_id);
        }
    }
}
