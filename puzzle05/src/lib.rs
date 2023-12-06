use std::ops::Range;

pub mod parser;
pub mod puzzle05a;
pub mod puzzle05b;

#[derive(Debug, PartialEq, Eq)]
struct MapFragment {
    domain: Range<i64>,
    offset: i64,
}

impl MapFragment {
    pub fn new(domain_start: i64, domain_length: i64, range_start: i64) -> Self {
        Self {
            domain: (domain_start..domain_start + domain_length),
            offset: range_start - domain_start,
        }
    }

    /// Computes the result of applying this fragment to an input value if the value
    /// is contained within its domain, and returns `None` otherwise.
    pub fn compute(&self, input: i64) -> Option<i64> {
        if self.domain.contains(&input) {
            Some(input + self.offset)
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Map(Vec<MapFragment>);

impl Map {
    pub fn new(fragments: Vec<MapFragment>) -> Self {
        Self(fragments)
    }

    /// Computes the result of applying this map to an input value by using the
    /// first fragment that contains the input in its domain. If no matching fragment is found,
    /// the input value is returned.
    pub fn compute(&self, input: i64) -> i64 {
        self.0
            .iter()
            .find_map(|fragment| fragment.compute(input))
            .unwrap_or(input)
    }

    /// This computes a single `Map` that is the result of applying two `Maps` in succession.
    pub fn compose(first: &Self, second: &Self) -> Self {
        // the "critical points" are where the domain of the composition needs
        // to be broken into subintervals.
        // critical points include the start/end points of the domain of the either Map,
        // as well as _preimages_ of the start/endpoints of the domain of the second Map under the first
        let mut critical_points: Vec<i64> = vec![];
        for MapFragment {
            domain: first_domain,
            offset,
        } in &first.0
        {
            critical_points.push(first_domain.start);
            critical_points.push(first_domain.end);

            let range = (first_domain.start + offset)..(first_domain.end + offset);

            for MapFragment {
                domain: second_domain,
                ..
            } in &second.0
            {
                // compute preimages of endpoints of the second Map's domain that lie
                // in the range of the first
                if range.contains(&second_domain.start) {
                    critical_points.push(second_domain.start - offset);
                }
                if range.contains(&second_domain.end) {
                    critical_points.push(second_domain.end - offset);
                }
            }
        }

        for MapFragment { domain, .. } in &second.0 {
            critical_points.push(domain.start);
            critical_points.push(domain.end);
        }

        // get organized
        critical_points.sort();
        critical_points.dedup();

        let mut fragments: Vec<MapFragment> = vec![];
        // now look at pairs of consecutive critical points and determine the offset for
        // the composition on each subinterval
        // because we've broken up the domain using critical points, this can be computed by
        // just looking at the first element in the interval
        critical_points.windows(2).for_each(|window| {
            if let [start, end] = window {
                // compute using the first map and then the second
                let output = second.compute(first.compute(*start));
                let offset = output - *start;

                fragments.push(MapFragment {
                    domain: (*start..*end),
                    offset,
                });
            }
        });

        Map(fragments)
    }

    fn smallest_output_over_interval(&self, interval: &Range<i64>) -> Option<i64> {
        // this relies on the fact that each fragment of the map is increasing, so its smallest
        // value on some set is always at the smallest input in that set
        //
        // sooo, we check every fragment's domain to see if it intersects the given interval.
        // if so, compute the output at the smallest input value in the intersection.
        // then take a min
        self.0
            .iter()
            .filter_map(|MapFragment { domain, offset }| {
                if interval.contains(&domain.start) {
                    Some(domain.start + offset)
                } else if domain.contains(&interval.start) {
                    Some(interval.start + offset)
                } else {
                    None
                }
            })
            .min()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct MapSet {
    seeds: Vec<Range<i64>>,
    maps: Vec<Map>,
}

impl MapSet {
    pub fn new(seeds: Vec<Range<i64>>, maps: Vec<Map>) -> Self {
        Self { seeds, maps }
    }

    pub fn seed_outputs(&self) -> Vec<i64> {
        self.seeds
            .iter()
            .flat_map(|seeds| {
                seeds
                    .clone()
                    .map(|seed| self.maps.iter().fold(seed, |value, map| map.compute(value)))
            })
            .collect()
    }

    // keeping this around for posterity
    #[allow(dead_code)]
    pub fn smallest_output_old(&self) -> i64 {
        *self.seed_outputs().iter().min().unwrap()
    }

    pub fn smallest_output(&self) -> i64 {
        // compute the composition of the whoooole set
        let composition: Map = self
            .maps
            .iter()
            .fold(Map::new(vec![]), |acc, curr| Map::compose(&acc, curr));

        // then use it to compute seed values
        self.seeds
            .iter()
            .filter_map(|interval| composition.smallest_output_over_interval(interval))
            .min()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_fragment_compute() {
        let fragment = MapFragment::new(50, 48, 52);

        assert_eq!(fragment.compute(79), Some(81));
        assert_eq!(fragment.compute(11), None);
    }

    #[test]
    fn test_map_compute() {
        let fragment1 = MapFragment::new(98, 2, 50);
        let fragment2 = MapFragment::new(50, 48, 52);

        let map = Map::new(vec![fragment1, fragment2]);

        assert_eq!(map.compute(79), 81);
        assert_eq!(map.compute(14), 14);
        assert_eq!(map.compute(99), 51);
    }

    #[test]
    fn test_map_set_compute() {
        let fragment1 = MapFragment::new(50, 48, 52);
        let map1 = Map::new(vec![fragment1]);

        let fragment2 = MapFragment::new(40, 30, 10);
        let map2 = Map::new(vec![fragment2]);

        let map_set = MapSet::new(vec![(60..62), (0..1)], vec![map1, map2]);

        assert_eq!(map_set.seed_outputs(), vec![32, 33, 0]);
    }

    #[test]
    fn test_map_compose_disjoint_range() {
        // (10..30) => +40    (range is 50..70)
        let fragment1 = MapFragment::new(10, 20, 50);
        let map1 = Map::new(vec![fragment1]);

        // (15..25) => -12    (neither endpoint in range)
        let fragment2 = MapFragment::new(15, 10, 3);
        let map2 = Map::new(vec![fragment2]);

        // (10..30) => +40    (same domain as first)
        let composition = Map::compose(&map1, &map2);
        let composition_outputs: Vec<_> = (0..30).map(|n| composition.compute(n)).collect();

        #[allow(clippy::single_range_in_vec_init)]
        let map_set = MapSet::new(vec![(0..30)], vec![map1, map2]);

        assert_eq!(composition_outputs, map_set.seed_outputs())
    }

    #[test]
    fn test_map_compose_range_contains_domain() {
        // (10..30) => +2    (range is 12..32)
        let fragment1 = MapFragment::new(10, 20, 12);
        let map1 = Map::new(vec![fragment1]);

        // (15..25) => -12   (both endpoints contained in range)
        let fragment2 = MapFragment::new(15, 10, 3);
        let map2 = Map::new(vec![fragment2]);

        // (10..13) => +2            (start of domain of first, preimage of start of domain of second)
        // (13..23) => +2 -12 = -10  (..., preimage of end of domain of second)
        // (23..30) => +2            (..., end of domain of first)
        let composition = Map::compose(&map1, &map2);
        let composition_outputs: Vec<_> = (0..30).map(|n| composition.compute(n)).collect();

        #[allow(clippy::single_range_in_vec_init)]
        let map_set = MapSet::new(vec![(0..30)], vec![map1, map2]);

        assert_eq!(composition_outputs, map_set.seed_outputs())
    }

    #[test]
    fn test_map_compose_range_overlaps_domain() {
        // (10..20) => +2   (range is 12..22)
        let fragment1 = MapFragment::new(10, 10, 12);
        let map1 = Map::new(vec![fragment1]);

        // (15..25) => -12  (first endpoint contained in range of first)
        let fragment2 = MapFragment::new(15, 10, 3);
        let map2 = Map::new(vec![fragment2]);

        // (10..13) => +2
        // (13..20) => +2 -12 = -10
        // (20..25) => -12
        let composition = Map::compose(&map1, &map2);
        let composition_outputs: Vec<_> = (0..30).map(|n| composition.compute(n)).collect();

        #[allow(clippy::single_range_in_vec_init)]
        let map_set = MapSet::new(vec![(0..30)], vec![map1, map2]);

        assert_eq!(composition_outputs, map_set.seed_outputs())
    }
}
