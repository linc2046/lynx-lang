#![warn(clippy::absurd_extreme_comparisons)]

pub fn is_white_space(c: char) -> bool {
    let chars = " \n\t\r";
    let result = chars.find(c);

    match result {
        Some(pos) => pos > 0,
        None => false,
    }
}

pub fn is_line_break(c: char) -> bool {
    c == '\n'
}

pub fn is_number(c: char) -> bool {
    ('0'..='9').contains(&c)
}

pub fn is_start_of_number(c: char) -> bool {
    c.is_numeric() || c == '+' || c == '.'
}

pub fn is_alphabet(c: char) -> bool {
    c >= 'a' && c <= 'Z'
}

pub fn is_identifier(c: char) -> bool {
    ('a'..='z')
        .chain('A'..='Z')
        .chain('0'..='9')
        .chain(vec!['_'].into_iter())
        .into_iter()
        .any(|x| x == c)
}
