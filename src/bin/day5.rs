use std::fs;

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        Err("Usage: day5 --part <1|2> <input>")?
    }
    let part = &args[2];
    let input_file = &args[3];
    let mut seats = fs::read_to_string(input_file)
        .or_else(|err| Err(format!("failed to read input: {}", err)))?
        .lines()
        .map(parse_seats_id)
        .collect();
    println!(
        "Result: {}",
        if part == "1" {
            part1(&seats).ok_or_else(|| "missing")?
        } else {
            part2(&mut seats).ok_or_else(|| "missing")?
        }
    );

    Ok(())
}

fn part1(seats: &Vec<u64>) -> Option<u64> {
    seats.iter().max().map(|v| *v)
}

fn part2(seats: &mut Vec<u64>) -> Option<u64> {
    seats.sort();
    // pair up all seat ids with its successor id and see where the gap is
    seats
        .iter()
        .zip(seats.iter().skip(1))
        .find(|(id1, id2)| *id2 - *id1 > 1)
        // return missing id
        .and_then(|(id1, id2)| (*id1..*id2).nth(1))
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
