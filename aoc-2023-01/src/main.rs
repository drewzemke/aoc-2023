use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
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

    println!("{}", sum);
}
