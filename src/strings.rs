/// Split a string on strings and group them in groups separated by blank lines.
///
/// # Examples
///
/// ```
/// use utils::strings;
///
/// let TEST_STR: &str = "
/// Hello,
/// World!
///
/// How are you?
///";
/// assert_eq!(strings::split_on_empty_lines(TEST_STR), vec![vec!["Hello,", "World!"], vec!["How are you?"]]);
/// ```
pub fn split_on_empty_lines<S>(input: S) -> Vec<Vec<String>>
where
    S: std::string::ToString,
{
    input.to_string().lines().fold(vec![], |mut acc, l| {
        if l == "" {
            acc.push(vec![]);
        } else if acc.last_mut().is_none() {
            acc.push(vec![l.to_string()]);
        } else {
            acc.last_mut().map(|ls| ls.push(l.to_string()));
        }
        acc
    })
}

/// Join a vector of lines into a String with newline characters (\n).
///
///
/// # Examples
///
/// ```
/// use utils::strings;
///
/// let LINES: Vec<&str> = vec!["Hello", "World", "!"];
/// assert_eq!(strings::join_lines(&LINES), "Hello\nWorld\n!");
/// ```
///
/// Joining an empty vec returns an empty string.
///
/// ```
/// use utils::strings;
///
/// let LINES: Vec<&str> = vec![];
/// assert_eq!(strings::join_lines(&LINES), "");
/// ```
pub fn join_lines<S>(lines: &Vec<S>) -> String
where
    S: std::string::ToString,
{
    lines.iter().fold(String::new(), |mut acc, s| {
        if acc.len() != 0 {
            acc.push_str("\n");
        }
        acc.push_str(s.to_string().as_str());
        acc
    })
}

///
/// # Examples
///
/// ```
/// use utils::strings;
///
/// let TEST_STR: &str = "Hello,
/// World!
///
/// How are you?
///";
/// assert_eq!(strings::split_on_empty_lines_string(TEST_STR), vec!["Hello,\nWorld!", "How are you?"]);
/// ```
pub fn split_on_empty_lines_string<S>(input: S) -> Vec<String>
where
    S: std::string::ToString,
{
    split_on_empty_lines(input).iter().map(join_lines).collect()
}
