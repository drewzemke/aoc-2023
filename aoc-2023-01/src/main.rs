use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    first_puzzle();
    second_puzzle();
}

fn first_puzzle() {
    let file = File::open("input").unwrap();
    let read = BufReader::new(file);

    let mut sum: u32 = 0;

    for line in read.lines().map(Result::unwrap) {
        let first_digit = line.chars().find_map(|c| c.to_digit(10)).unwrap();
        let last_digit = line
            .chars()
            .rfind(|c| c.is_ascii_digit())
            .unwrap()
            .to_digit(10)
            .unwrap();

        let line_value = first_digit * 10 + last_digit;
        sum += line_value;
    }

    println!("first puzzle: {}", sum);
}

fn second_puzzle() {
    let file = File::open("input").unwrap();
    let read = BufReader::new(file);

    let mut sum: u32 = 0;

    for line in read.lines().map(Result::unwrap) {
        let digits = find_digits(&line);
        let first_digit = digits.first().unwrap();
        let last_digit = digits.last().unwrap();

        let line_value = first_digit * 10 + last_digit;
        sum += line_value;
    }

    println!("second puzzle: {}", sum);
}

fn find_digits(s: &str) -> Vec<u32> {
    let mut digits = vec![];
    let mut haystack = s;
    while !haystack.is_empty() {
        if let (Some(digit), remainder) = parse_numerical_digit(haystack) {
            digits.push(digit);
            haystack = remainder;
            continue;
        }
        if let (Some(digit), _) = parse_word_digit(haystack) {
            digits.push(digit);
            // can't do `haystack = remainder` here because it might skip over
            // some matches (eg. "twone")
            haystack = &haystack[1..];
            continue;
        } else {
            haystack = &haystack[1..];
        }
    }

    digits
}

fn parse_numerical_digit(s: &str) -> (Option<u32>, &str) {
    s.chars()
        .next()
        .and_then(|c| c.to_digit(10))
        .map_or((None, s), |d| (Some(d), &s[1..]))
}

const WORD_DIGITS: [(&str, u32); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn parse_word_digit(s: &str) -> (Option<u32>, &str) {
    for (word, digit) in WORD_DIGITS {
        if let Some(remainder) = s.strip_prefix(word) {
            return (Some(digit), remainder);
        }
    }
    (None, s)
}
