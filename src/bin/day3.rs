use std::fs;

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        Err("Usage: day3 --part <1|2> <input>")?
    }
    let part = &args[2];
    let input_file = &args[3];
    let map = fs::read_to_string(input_file)
        .or_else(|err| Err(format!("failed to read input: {}", err)))
        .and_then(|s| Ok(Map::parse(s)))?;
    if part == "1" {
        println!("Result: {}", part1(map));
    } else {
        println!("Result: {}", part2(map));
    }

    Ok(())
}

fn part1(map: Map) -> u32 {
    let (end_row, end_column) = map.goal_position;
    let rows = 0..end_row;
    let columns = (0..end_row).map(|c| c * 3 % end_column).take(end_row);
    rows.zip(columns).fold(0, |acc, (x, y)| {
        acc + if map.has_tree(x, y).unwrap() { 1 } else { 0 }
    })
}

fn part2(_map: Map) -> u32 {
    unimplemented!()
}

#[derive(Debug)]
struct Map {
    rows: Vec<Vec<char>>,
    goal_position: (usize, usize),
}

impl Map {
    fn parse(input: String) -> Map {
        let rows: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
        let last_row = rows.len();
        let last_column = rows[last_row - 1].len();
        Map {
            rows: rows,
            goal_position: (last_row, last_column),
        }
    }

    fn has_tree(&self, x: usize, y: usize) -> Result<bool, String> {
        if x + 1 > self.rows.len() || y + 1 > self.rows[x].len() {
            Err(format!(
                "({}, {}) is outside of map (dimensions: {} x {})",
                x,
                y,
                self.rows.len(),
                self.rows[0].len()
            ))
        } else {
            Ok(self.rows[x][y] == '#')
        }
    }
}
