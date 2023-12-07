pub mod puzzle06a;
pub mod puzzle06b;

mod parser {
    fn parse_line(input: &str) -> Vec<u32> {
        let (_, nums) = input.split_once(":").unwrap();
        nums.split_whitespace()
            .map(|str| str.parse::<u32>().unwrap())
            .collect()
    }

    pub fn parse(input: &str) -> Vec<(u32, u32)> {
        // there should be only two lines
        let line1 = input.lines().next().unwrap();
        let line2 = input.lines().nth(1).unwrap();

        let times = parse_line(line1);
        let distances = parse_line(line2);
        times.into_iter().zip(distances).collect()
    }

    #[test]
    fn test_parse_line() {
        assert_eq!(parse_line("Time:      7  15   30 "), vec![7, 15, 30]);
        assert_eq!(
            parse_line("Distance:   291   1172   1176   2026"),
            vec![291, 1172, 1176, 2026,]
        );
    }
}
