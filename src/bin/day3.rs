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

fn part1(map: Map) -> u64 {
    map.trees_in_path(3, 1)
}

fn part2(map: Map) -> u64 {
    let paths: Vec<(usize, usize)> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    paths
        .iter()
        .map(|(right, down)| map.trees_in_path(*right, *down))
        .product()
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

    fn trees_in_path(&self, right_increment: usize, down_increment: usize) -> u64 {
        let (end_row, end_column) = self.goal_position;
        let rows = (0..end_row)
            .map(|r| r * down_increment)
            .take_while(|r| *r < end_row);
        let columns = (0..end_row)
            .map(|c| c * right_increment % end_column)
            .take(end_row);
        rows.zip(columns).fold(0, |acc, (x, y)| {
            acc + if self.has_tree(x, y).unwrap() { 1 } else { 0 }
        })
    }
}
