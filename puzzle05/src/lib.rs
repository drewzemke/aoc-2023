use std::ops::Range;

pub mod parser;
pub mod puzzle05a;
pub mod puzzle05b;

#[derive(Debug, PartialEq, Eq)]
struct MapFragment {
    source: Range<u64>,
    dest_offset: u64,
}

impl MapFragment {
    /// Computes the result of applying this fragment to an input value if the value
    /// is contained within its domain, and returns `None` otherwise.
    pub fn compute(&self, input: u64) -> Option<u64> {
        if self.source.contains(&input) {
            Some(self.dest_offset + (input - self.source.start))
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Map(Vec<MapFragment>);

impl Map {
    /// Computes the result of applying this map to an input value by using the
    /// first fragment that contains the input in its domain. If no matching fragment is found,
    /// the input value is returned.
    pub fn compute(&self, input: u64) -> u64 {
        self.0
            .iter()
            .find_map(|fragment| fragment.compute(input))
            .unwrap_or(input)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct MapSet {
    seeds: Vec<u64>,
    maps: Vec<Map>,
}

impl MapSet {
    pub fn seed_outputs(&self) -> Vec<u64> {
        self.seeds
            .iter()
            .map(|seed| {
                self.maps
                    .iter()
                    .fold(*seed, |value, map| map.compute(value))
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_fragment_compute() {
        let fragment = MapFragment {
            source: (50..98),
            dest_offset: 52,
        };

        assert_eq!(fragment.compute(79), Some(81));
        assert_eq!(fragment.compute(11), None);
    }

    #[test]
    fn test_map_compute() {
        let fragment1 = MapFragment {
            source: (98..100),
            dest_offset: 50,
        };
        let fragment2 = MapFragment {
            source: (50..98),
            dest_offset: 52,
        };

        let map = Map(vec![fragment1, fragment2]);

        assert_eq!(map.compute(79), 81);
        assert_eq!(map.compute(14), 14);
        assert_eq!(map.compute(99), 51);
    }

    #[test]
    fn test_map_set_compute() {
        let fragment1 = MapFragment {
            source: (50..98),
            dest_offset: 52,
        };

        let map1 = Map(vec![fragment1]);

        let fragment2 = MapFragment {
            source: (40..70),
            dest_offset: 10,
        };

        let map2 = Map(vec![fragment2]);

        let map_set = MapSet {
            seeds: vec![60, 0],
            maps: vec![map1, map2],
        };

        assert_eq!(map_set.seed_outputs(), vec![32, 0]);
    }
}
