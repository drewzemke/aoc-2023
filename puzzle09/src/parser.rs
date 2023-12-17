use crate::DiscreteFn;

pub fn parse_line(line: &str) -> DiscreteFn {
    let nums = line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    DiscreteFn(nums)
}

#[cfg(test)]
mod parse_tests {
    use crate::{parser::parse_line, DiscreteFn};

    #[test]
    fn test_parse_disc_fn() {
        let input = "0 3 6 9 12 15";
        assert_eq!(parse_line(input), DiscreteFn(vec![0, 3, 6, 9, 12, 15]))
    }
}
