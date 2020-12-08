use std::collections::{HashMap, HashSet};
use std::str::FromStr;

fn main() -> Result<(), String> {
    let (part, content) = utils::parse_args()?;
    let rules: Rules = content.parse()?;
    utils::run(part1, part2, part, rules);
    Ok(())
}

fn part1(r: Rules) -> u32 {
    let b: Bag = "shiny gold".parse().expect("should not fail");
    r.bags
        .keys()
        .filter(|k| r.bag_can_contain_bag(&k, &b))
        .count() as u32
}

fn part2(r: Rules) -> u32 {
    let b: Bag = "shiny gold".parse().expect("should not fail");
    fn count_bags(rules: &Rules, b: &Bag) -> u32 {
        1 + match rules.bags.get(b) {
            None => 0,
            Some(v) => v.iter().map(|(k, n)| count_bags(rules, k) * n).sum(),
        }
    }
    count_bags(&r, &b) - 1
}

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
/// Represents a bag rule.
struct Bag {
    adjective: String,
    color: String,
}

impl FromStr for Bag {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split(' ');
        Ok(Bag {
            adjective: words.next().map_or_else(
                || Err("expected an adjective".to_string()),
                |a| Ok(a.to_string()),
            )?,
            color: words.next().map_or_else(
                || Err("expected a color".to_string()),
                |c| Ok(c.to_string()),
            )?,
        })
    }
}

#[derive(Clone, Debug)]
/// Represents all the rules for bag configurations in the luggage
/// regulation.
///
/// Rules form a digraph.
struct Rules {
    bags: HashMap<Bag, Vec<(Bag, u32)>>,
}

impl Rules {
    fn bag_can_contain_bag(&self, b1: &Bag, b2: &Bag) -> bool {
        self.bags.get(b1).map_or_else(
            || false,
            |bs| {
                bs.iter().any(|(b, _)| {
                    if b == b2 {
                        true
                    } else {
                        self.bag_can_contain_bag(b, b2)
                    }
                })
            },
        )
    }

    fn parse_rule_line(l: &str, rules: &mut HashMap<Bag, Vec<(Bag, u32)>>) -> Result<(), String> {
        let mut words = l.split(' ');
        let bag: Bag = format!("{} {}", words.next().unwrap(), words.next().unwrap()).parse()?;
        expect_str(words.next(), "bags")?;
        expect_str(words.next(), "contain")?;
        let mut contained_bags: Vec<(Bag, u32)> = vec![];
        loop {
            let next = words.next();
            match next {
                Some("no") => {
                    expect_str(words.next(), "other")?;
                    expect_str(words.next(), "bags.")?;
                    break;
                }
                Some(_) => (),
                None => Err("expected either 'no other bags' or a number".to_string())?,
            };
            let amount: u32 = next
                .unwrap()
                .parse()
                .map_err(|e| format!("failed to parse number: {}", e))?;
            let contained_bag: Bag =
                format!("{} {}", words.next().unwrap(), words.next().unwrap()).parse()?;
            contained_bags.push((contained_bag, amount));
            if words.next().map(|w| w.contains('.')).unwrap_or(true) {
                // last word ends with a .
                break;
            }
        }
        rules.insert(bag, contained_bags);
        Ok(())
    }
}

impl FromStr for Rules {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bags: Result<HashMap<Bag, Vec<(Bag, u32)>>, String> =
            s.lines().try_fold(HashMap::new(), |mut acc, l| {
                match Rules::parse_rule_line(l, &mut acc) {
                    Ok(_) => (),
                    Err(e) => return Err(format!("failed to parse line {} as rule: {}", l, e)),
                };
                Ok(acc)
            });
        bags.map(|b| Rules { bags: b })
    }
}

fn expect_str<S>(o: Option<S>, expected: &str) -> Result<(), String>
where
    S: std::string::ToString + std::fmt::Display,
{
    match o {
        None => Err(format!("expected '{}'", expected)),
        Some(s) => {
            if s.to_string() == expected.to_string() {
                Ok(())
            } else {
                Err(format!("expected {}, but got {}", expected, s))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    #[test]
    fn test_bag_can_contain_bag() {
        let rules: Rules = "bright white bags contain 1 shiny gold bag."
            .parse()
            .unwrap();
        let b = Bag {
            adjective: String::from("bright"),
            color: String::from("white"),
        };
        let b2 = Bag {
            adjective: String::from("shiny"),
            color: String::from("gold"),
        };
        assert_eq!(rules.bags.get(&b), Some(&vec![(b2.clone(), 1)]));
        assert_eq!(rules.bag_can_contain_bag(&b, &b2), true);
    }

    #[test]
    fn test_part_one() {
        let rules: Rules = TEST_INPUT.parse().expect("parsing should not fail");
        assert_eq!(part1(rules), 4);
    }

    #[test]
    fn test_part_two() {
        let rules: Rules = "shiny gold bags contain 1 bright white bag, 2 bright yellow bags.
bright white bags contain 1 bright yellow bag."
            .parse()
            .unwrap();
        assert_eq!(part2(rules), 1 + 1 + 2 * 1);
    }

    #[test]
    fn test_part_two_test_string() {
        let rules: Rules = TEST_INPUT.parse().expect("parsing should not fail");
        assert_eq!(part2(rules), 32);
    }

    #[test]
    fn test_part_two_test_string2() {
        let rules: Rules = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags."
            .parse()
            .expect("parsing should not fail");
        assert_eq!(part2(rules), 126);
    }
}
