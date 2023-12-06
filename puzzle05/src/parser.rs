use std::ops::Range;

use crate::{Map, MapFragment, MapSet};

impl MapFragment {
    /// Assumes that input is a single line consisting of three numbers separated by whitespace.
    pub fn parse_from_str(input: &str) -> Self {
        let mut number_strs = input.split_whitespace();

        let dest_offset = number_strs.next().unwrap().parse::<i64>().unwrap();
        let source_start = number_strs.next().unwrap().parse::<i64>().unwrap();
        let source_length = number_strs.next().unwrap().parse::<i64>().unwrap();

        MapFragment::new(source_start, source_length, dest_offset)
    }
}

impl Map {
    /// Assumes the input is a set of lines, each of which consists of three numbers.
    #[allow(dead_code)]
    fn parse_from_str(input: &str) -> Self {
        let fragments = input.lines().map(MapFragment::parse_from_str).collect();
        Map::new(fragments)
    }
}

pub enum SeedParseStrategy {
    IndividualSeeds,
    PairedRanges,
}

impl MapSet {
    /// Assumes the input contains the prefix "seeds: " followed by a list of whitespace-separated values.
    fn parse_individual_seeds(input: &str) -> Vec<Range<i64>> {
        input
            .strip_prefix("seeds: ")
            .unwrap()
            .split_whitespace()
            .map(|s| {
                let start = s.parse::<i64>().unwrap();
                start..start + 1
            })
            .collect()
    }

    /// Assumes the input contains the prefix "seeds: " followed by a list of whitespace-separated values.
    fn parse_seed_range_pairs(input: &str) -> Vec<Range<i64>> {
        input
            .strip_prefix("seeds: ")
            .unwrap()
            .split_whitespace()
            .collect::<Vec<_>>()
            .chunks_exact(2)
            .map(|chunk| match chunk {
                [s, t] => {
                    let start = s.parse::<i64>().unwrap();
                    let length = t.parse::<i64>().unwrap();
                    start..start + length
                }
                _ => unreachable!(),
            })
            .collect()
    }

    /// Assumes that the input is in the form of the entire input for this file:
    /// - starts with a "seed: " line
    /// - remainder consists of blocks each with a header line, the rest of which describes a map
    pub fn parse_from_str(input: &str, seed_strategy: SeedParseStrategy) -> Self {
        let mut lines = input.lines();
        let seeds = match seed_strategy {
            SeedParseStrategy::IndividualSeeds => {
                MapSet::parse_individual_seeds(lines.next().unwrap())
            }
            SeedParseStrategy::PairedRanges => {
                MapSet::parse_seed_range_pairs(lines.next().unwrap())
            }
        };

        // skip the next empty line
        let lines = lines.skip(1);

        let mut current_map_frags: Vec<MapFragment> = vec![];
        let mut maps: Vec<Map> = vec![];

        // TODO: try using itertools->chunks
        for line in lines {
            // if we hit a whitespace line, it's the end of this block, so add the map
            // we're working on to the list
            if line.trim().is_empty() {
                maps.push(Map(current_map_frags));
                current_map_frags = vec![];
                continue;
            }

            // ignore lines that don't start with digits (ie. the header lines)
            if line.trim().chars().next().unwrap().is_alphabetic() {
                continue;
            }

            // otherwise parse a map fragment
            current_map_frags.push(MapFragment::parse_from_str(line));
        }
        maps.push(Map(current_map_frags));

        MapSet::new(seeds, maps)
    }
}

#[cfg(test)]
mod tests {
    use crate::{parser::SeedParseStrategy, Map, MapFragment, MapSet};

    #[test]
    fn test_parse_fragment() {
        let input = "50 98 2";

        assert_eq!(
            MapFragment::parse_from_str(input),
            MapFragment::new(98, 2, 50)
        );
    }

    #[test]
    fn test_parse_map() {
        let input = r"50 98 2 
        52 50 48";

        assert_eq!(
            Map::parse_from_str(input),
            Map::new(vec![
                MapFragment::new(98, 2, 50),
                MapFragment::new(50, 48, 52),
            ])
        );
    }

    #[test]
    fn test_parse_individual_seeds() {
        let input = "seeds: 79 14 55 13";

        assert_eq!(
            MapSet::parse_individual_seeds(input),
            vec![79..80, 14..15, 55..56, 13..14]
        );
    }

    #[test]
    fn test_parse_seed_ranges() {
        let input = "seeds: 79 14 55 13";

        assert_eq!(MapSet::parse_seed_range_pairs(input), vec![79..93, 55..68]);
    }

    #[test]
    fn test_parse_map_set_individual_seeds() {
        let input = r"seeds: 79 14 55 13

            seed-to-soil map:
            50 98 2
            52 50 48

            soil-to-fertilizer map:
            0 15 37
            37 52 2";

        assert_eq!(
            MapSet::parse_from_str(input, SeedParseStrategy::IndividualSeeds),
            MapSet::new(
                vec![79..80, 14..15, 55..56, 13..14],
                vec![
                    Map::new(vec![
                        MapFragment::new(98, 2, 50),
                        MapFragment::new(50, 48, 52),
                    ]),
                    Map::new(vec![
                        MapFragment::new(15, 37, 0),
                        MapFragment::new(52, 2, 37),
                    ]),
                ]
            )
        );
    }

    #[test]
    fn test_parse_map_set_paired_seeds() {
        let input = r"seeds: 79 14 55 13

            seed-to-soil map:
            50 98 2
            52 50 48

            soil-to-fertilizer map:
            0 15 37
            37 52 2";

        assert_eq!(
            MapSet::parse_from_str(input, SeedParseStrategy::PairedRanges),
            MapSet::new(
                vec![79..93, 55..68],
                vec![
                    Map::new(vec![
                        MapFragment::new(98, 2, 50),
                        MapFragment::new(50, 48, 52),
                    ]),
                    Map::new(vec![
                        MapFragment::new(15, 37, 0),
                        MapFragment::new(52, 2, 37),
                    ]),
                ]
            ),
        );
    }
}
