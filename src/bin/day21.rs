use std::{collections::HashMap, collections::HashSet, str::FromStr};

fn main() -> Result<(), String> {
    let (part, content) = utils::parse_args()?;
    let lines = content
        .lines()
        .map(|l| {
            l.parse()
                .map_err(|e| format!("failed to parse {} as a Line: {}", l, e))
        })
        .collect::<Result<Vec<Line>, String>>()?;
    utils::run(part1, part2, part, lines);
    Ok(())
}

fn part1(input: Vec<Line>) -> u32 {
    let mut possible_allergens: HashMap<String, Vec<String>> = HashMap::new();
    let mut determined_allergens: HashMap<String, String> = HashMap::new();
    let mut all_foods: Vec<String> = Vec::new();
    for Line { foods, allergens } in input {
        &foods.iter().for_each(|f| {
            all_foods.push(String::from(f));
        });
        for allergen in allergens {
            if determined_allergens
                .iter()
                .any(|(_, a)| a.as_str() == &allergen)
            {
                continue;
            }
            let entry = possible_allergens
                .entry(allergen.clone())
                .or_insert(Vec::new());
            foods.iter().for_each(|f| {
                let _ = entry.push(f.to_string());
            })
        }
    }
    for _ in 0..10 {
        println!(
            "pos: {:?}, det: {:?}",
            &possible_allergens, determined_allergens
        );
        let found_allergens: Vec<(String, String)> = possible_allergens
            .iter()
            .filter(|(_, is)| is.len() == 1)
            .map(|(a, is)| (a.to_string(), is.iter().next().unwrap().to_string()))
            .collect();

        found_allergens.iter().for_each(|(a, i)| {
            let _ = determined_allergens.insert(i.to_string(), a.to_string());
            possible_allergens.remove(a);
            possible_allergens.iter().map(|(a, is)| {
                (
                    a,
                    is.iter()
                        .cloned()
                        .filter(|i2| i2 != i)
                        .collect::<Vec<String>>(),
                )
            });
        })
    }
    all_foods
        .iter()
        .cloned()
        .filter(|f| !determined_allergens.contains_key(f))
        .count() as u32
}

fn part2(_input: Vec<Line>) -> u32 {
    unimplemented!()
}

#[derive(Clone, Debug, PartialEq)]
struct Line {
    foods: Vec<String>,
    allergens: Vec<String>,
}

impl FromStr for Line {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let foods = s
            .split(' ')
            .take_while(|w| w.chars().next() != Some('('))
            .map(|w| String::from(w))
            .collect();
        let allergens = s
            .split(' ')
            .skip_while(|w| w.chars().next() != Some('('))
            .map(|w| w.trim_matches(|c| c == '(' || c == ')' || c == ','))
            .skip(1)
            .map(|w| String::from(w.trim_end_matches(',')))
            .collect();
        Ok(Line { foods, allergens })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let expected = Line {
            foods: vec![
                String::from("mxmxvkd"),
                String::from("kfcds"),
                String::from("sqjhc"),
                String::from("nhms"),
            ],
            allergens: vec![String::from("dairy"), String::from("fish")],
        };
        assert_eq!(
            TEST_STR.lines().next().map(|l| l.parse::<Line>()),
            Some(Ok(expected))
        );
    }

    #[test]
    fn test_part1() {
        let lines = TEST_STR
            .lines()
            .map(|l| {
                l.parse()
                    .map_err(|e| format!("failed to parse {} as a Line: {}", l, e))
            })
            .collect::<Result<Vec<Line>, String>>()
            .unwrap();
        assert_eq!(part1(lines), 5);
    }

    const TEST_STR: &str = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";
}
