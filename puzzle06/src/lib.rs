pub mod puzzle06a;
pub mod puzzle06b;

mod math_things {
    // If the race is T seconds long, and you charge it for x seconds, then
    // it will travel at x mm/s for (T-x) seconds, meaning it travels
    // a total of x * (T-x) mm.
    // We want to find the number of integer values of x for which the
    // distance traveled is greater than some distance D:
    //     x * (T-x) > D
    // This is a quadratic equation:
    //     x^2 - Tx + D < 0
    // Its solutions are the integer values for which:
    //     (T - sqrt(T^2 - 4*D)) / 2  <  x  < (T + sqrt(T^2 - 4*D)) / 2
    // If those two numbers are not integers themselves, the number of integers
    // between them is the difference of their floors.
    // If they are integers, the number of integers between them is one less
    // than their differences.
    // Sooo let's just do all that!
    pub fn number_of_ways_to_win(time: u64, distance: u64) -> u64 {
        let disc = (time * time - 4 * distance) as f64;
        let disc_sqrt = disc.powf(0.5);
        let left = (time as f64 - disc_sqrt) / 2.0;
        let right = (time as f64 + disc_sqrt) / 2.0;

        // if disc_sqrt is an integer...
        if disc_sqrt.floor() == disc_sqrt {
            (right - left) as u64 - 1
        } else {
            (right.floor() as u64) - (left.floor() as u64)
        }
    }

    #[test]
    fn test_number_of_way_to_win() {
        assert_eq!(number_of_ways_to_win(7, 9), 4);
        assert_eq!(number_of_ways_to_win(15, 40), 8);
        assert_eq!(number_of_ways_to_win(30, 200), 9);
    }
}

mod parser {

    fn parse_line(input: &str) -> Vec<u64> {
        let (_, nums) = input.split_once(':').unwrap();
        nums.split_whitespace()
            .map(|str| str.parse::<u64>().unwrap())
            .collect()
    }

    fn parse_line_ignore_spaces(input: &str) -> u64 {
        let (_, nums) = input.split_once(':').unwrap();
        nums.split_whitespace()
            .collect::<String>()
            .parse::<u64>()
            .unwrap()
    }

    pub fn parse(input: &str) -> Vec<(u64, u64)> {
        // there should be only two lines
        let line1 = input.lines().next().unwrap();
        let line2 = input.lines().nth(1).unwrap();

        let times = parse_line(line1);
        let distances = parse_line(line2);
        times.into_iter().zip(distances).collect()
    }

    pub fn parse_ignore_spaces(input: &str) -> (u64, u64) {
        // there should be only two lines
        let line1 = input.lines().next().unwrap();
        let line2 = input.lines().nth(1).unwrap();

        let times = parse_line_ignore_spaces(line1);
        let distances = parse_line_ignore_spaces(line2);
        (times, distances)
    }

    #[test]
    fn test_parse_line() {
        assert_eq!(parse_line("Time:      7  15   30 "), vec![7, 15, 30]);
        assert_eq!(
            parse_line("Distance:   291   1172   1176   2026"),
            vec![291, 1172, 1176, 2026,]
        );
    }

    #[test]
    fn test_parse_line_ignore_spaces() {
        assert_eq!(parse_line_ignore_spaces("Time:      7  15   30 "), 71530);
        assert_eq!(
            parse_line_ignore_spaces("Distance:   291   1172   1176   2026"),
            291117211762026
        );
    }
}
