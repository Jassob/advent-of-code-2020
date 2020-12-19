use std::{collections::HashMap, str::FromStr};

fn main() -> Result<(), String> {
    let (part, content) = utils::parse_args()?;
    utils::run(part1, part2, part, content);
    Ok(())
}

fn part1(input: String) -> u32 {
    let rules: String = input
        .lines()
        .take_while(|l| *l != "")
        .map(|l| l.to_string())
        .collect::<Vec<String>>()
        .join("\n");
    let m: Matcher = rules.parse().unwrap();
    input
        .lines()
        .skip_while(|l| *l != "")
        .skip(1)
        .filter(|l| {
            let (s, b) = m.check(*l);
            s == "" && b
        })
        .count() as u32
}

fn part2(_input: String) -> u32 {
    unimplemented!()
}

#[derive(Clone, Debug, PartialEq)]
enum Matcher {
    Empty,
    Literal(char),
    Sequence(Box<Matcher>, Box<Matcher>),
    Alternative(Box<Matcher>, Box<Matcher>),
}

fn check_for(c: char) -> Matcher {
    Matcher::Literal(c)
}

impl Matcher {
    fn check<'a>(&self, s: &'a str) -> (&'a str, bool) {
        match self {
            Matcher::Empty => (s, false),
            Matcher::Literal(c) => {
                if s.chars().next() == Some(*c) {
                    return (&s[1..], true);
                }
                (s, false)
            }
            Matcher::Sequence(first, then) => {
                if let (s1, true) = first.check(s) {
                    if let (s2, true) = then.check(s1) {
                        return (s2, true);
                    }
                }
                (s, false)
            }
            Matcher::Alternative(this, that) => {
                if let (s1, true) = this.check(s) {
                    return (s1, true);
                } else if let (s2, true) = that.check(s) {
                    return (s2, true);
                }
                (s, false)
            }
        }
    }

    fn then(self, matcher: Matcher) -> Matcher {
        Matcher::Sequence(Box::new(self), Box::new(matcher))
    }

    fn or(self, matcher: Matcher) -> Matcher {
        Matcher::Alternative(Box::new(self), Box::new(matcher))
    }

    fn parse_body(bodies: &HashMap<u32, String>, id: u32) -> Result<Self, String> {
        let body = bodies
            .get(&id)
            .ok_or_else(|| format!("could not find body for rule {}", id))?;
        let mut matchers = vec![Matcher::Empty];
        let mut memoized_matchers: HashMap<u32, Matcher> = HashMap::new();

        fn combine_matcher(m1: Matcher, m2: Matcher) -> Matcher {
            if m1 == Matcher::Empty {
                m1.or(m2)
            } else {
                m1.then(m2)
            }
        }

        for w in body.trim().split(' ') {
            if w == "|" {
                matchers.push(Matcher::Empty);
            } else if w.len() > 2 && &w[0..1] == "\"" {
                let c = w.bytes().skip(1).next().ok_or_else(|| {
                    format!(
                        "expected a single character inside quotes, but found: {}",
                        w
                    )
                })?;
                matchers
                    .last_mut()
                    .map(|curr| *curr = combine_matcher(curr.clone(), check_for(c as char)));
            } else {
                let id = w
                    .parse::<u32>()
                    .map_err(|e| format!("could not parse {} as a matcher id: {}", w, e))?;
                match memoized_matchers.get(&id) {
                    Some(m) => {
                        matchers
                            .last_mut()
                            .map(|curr| *curr = combine_matcher(curr.clone(), m.clone()));
                    }
                    None => {
                        let m = Matcher::parse_body(bodies, id)?;
                        memoized_matchers.insert(id, m.clone());
                        matchers
                            .last_mut()
                            .map(|curr| *curr = combine_matcher(curr.clone(), m));
                    }
                }
            }
        }

        Ok(matchers
            .iter()
            .fold(Matcher::Empty, |acc, m| acc.or(m.clone())))
    }
}

impl FromStr for Matcher {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let matchers = s
            .lines()
            .map(|l| -> Result<(u32, String), String> {
                let mut parts = l.split(':');
                let id = parts.next().map_or_else(
                    || Err("expected an id".to_string()),
                    |s| {
                        s.parse::<u32>()
                            .map_err(|e| format!("failed to parse {} as a number: {}", s, e))
                    },
                )?;
                let body: String = parts
                    .next()
                    .ok_or_else(|| "expected a body".to_string())?
                    .to_string();
                Ok((id, body))
            })
            .collect::<Result<HashMap<u32, String>, String>>()?;
        Matcher::parse_body(&matchers, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST1: &str = "0: 1 2
1: \"a\"
2: 1 3 | 3 1
3: \"b\"";

    #[test]
    fn test_check_literal() {
        let m = check_for('a');
        assert_eq!(m.check("a"), ("", true));
        assert_eq!(m.check("b"), ("b", false));
    }

    #[test]
    fn test_check_or() {
        let m = check_for('a')
            .then(check_for('b'))
            .or(check_for('b').then(check_for('a')));
        assert_eq!(m.check("ab"), ("", true));
        assert_eq!(m.check("ba"), ("", true));
        assert_eq!(m.check("a"), ("a", false));
    }

    #[test]
    fn test_check() {
        let matcher = check_for('a')
            .then((check_for('a').then(check_for('b'))).or(check_for('b').then(check_for('a'))));

        assert_eq!(matcher.check("aab"), ("", true));
        assert_eq!(matcher.check("aba"), ("", true));
        assert_eq!(matcher.check("apa"), ("apa", false));
    }

    #[test]
    fn test_part1() {
        let matcher: Matcher = TEST1.parse().unwrap();
        println!("{:?}", matcher);
        assert_eq!(matcher.check("aab"), ("", true));
    }
}
