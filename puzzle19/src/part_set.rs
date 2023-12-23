use crate::{Category, Operator};
use std::ops::Range;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PartSet {
    pub x: Range<u64>,
    pub m: Range<u64>,
    pub a: Range<u64>,
    pub s: Range<u64>,
}

impl PartSet {
    pub fn get(&self, category: &Category) -> &Range<u64> {
        match category {
            Category::X => &self.x,
            Category::M => &self.m,
            Category::A => &self.a,
            Category::S => &self.s,
        }
    }

    pub fn clone_with(&self, category: &Category, range: Range<u64>) -> PartSet {
        match category {
            Category::X => PartSet {
                x: range,
                ..self.clone()
            },
            Category::M => PartSet {
                m: range,
                ..self.clone()
            },
            Category::A => PartSet {
                a: range,
                ..self.clone()
            },
            Category::S => PartSet {
                s: range,
                ..self.clone()
            },
        }
    }

    /// Creates two disjoint sets from this one: one that satisfies the rule
    /// (that the parts' values in `category` must relate to `value` by `operator`)
    /// and one that does not
    pub fn split(
        &self,
        category: &Category,
        operator: &Operator,
        value: u64,
    ) -> (Option<PartSet>, Option<PartSet>) {
        let range = self.get(category);

        // check if this entire set matches
        if (*operator == Operator::GreaterThan && range.start > value)
            || (*operator == Operator::LessThan && range.end <= value)
        {
            return (Some(self.clone()), None);
        }

        // check if this entire set does not match
        if (*operator == Operator::GreaterThan && range.end <= value)
            || (*operator == Operator::LessThan && range.start > value)
        {
            return (None, Some(self.clone()));
        }

        let (match_range, unmatch_range) = match operator {
            Operator::GreaterThan => (value + 1..range.end, range.start..value + 1),
            Operator::LessThan => (range.start..value, value..range.end),
        };

        (
            Some(self.clone_with(category, match_range)),
            Some(self.clone_with(category, unmatch_range)),
        )
    }

    /// Decompose a set into a collection of subsets that are
    /// pairwise disjoint from themselves and from this set
    pub fn refine(&self, other: &PartSet) -> Vec<PartSet> {
        let mut refinements = vec![other.clone()];

        while !refinements.iter().all(|set| self.is_disjoint_from(set)) {
            for category in [Category::X, Category::M, Category::A, Category::S] {
                let self_range = self.get(&category);
                let other_range = other.get(&category);

                refinements = refinements
                    .into_iter()
                    .flat_map(|set| {
                        let mut sets = vec![];

                        // self:     (      )
                        // other: (      )
                        if other_range.contains(&(self_range.start - 1))
                            && !other_range.contains(&self_range.end)
                        {
                            sets.push(
                                set.clone_with(&category, other_range.start..self_range.start),
                            );
                            sets.push(set.clone_with(&category, self_range.start..other_range.end));
                        }
                        // self:  (      )
                        // other:     (      )
                        else if other_range.contains(&self_range.end)
                            && !other_range.contains(&(self_range.start - 1))
                        {
                            sets.push(set.clone_with(&category, other_range.start..self_range.end));
                            sets.push(set.clone_with(&category, self_range.end..other_range.end));
                        }
                        // self:      (      )
                        // other:  (             )
                        else if other_range.contains(&self_range.end)
                            && other_range.contains(&self_range.start)
                        {
                            sets.push(
                                set.clone_with(&category, other_range.start..self_range.start),
                            );
                            sets.push(set.clone_with(&category, self_range.start..self_range.end));
                            sets.push(set.clone_with(&category, self_range.end..other_range.end));
                        // }
                        // self:  (             )
                        // other:     (      )
                        // else if self_range.contains(&(other_range.end - 1))
                        //     && self_range.contains(&other_range.start)
                        // {
                        // FIXME: I think this is wrong
                        // do nothing, just eat this one later?
                        } else {
                            sets.push(set);
                        }

                        sets
                    })
                    .collect();
            }

            // remove all of the sets of the refinement that are proper subsets
            // of this one
            refinements.retain(|set| {
                ![Category::X, Category::M, Category::A, Category::S]
                    .iter()
                    .all(|category| {
                        let self_range = self.get(category);
                        let other_range = set.get(category);
                        self_range.contains(&other_range.start)
                            && self_range.contains(&(other_range.end - 1))
                    })
            });
        }

        refinements
    }

    pub fn size(&self) -> u64 {
        (self.x.end - self.x.start)
            * (self.m.end - self.m.start)
            * (self.a.end - self.a.start)
            * (self.s.end - self.s.start)
    }

    pub fn is_disjoint_from(&self, other: &PartSet) -> bool {
        [Category::X, Category::M, Category::A, Category::S]
            .iter()
            .any(|category| {
                let self_range = self.get(category);
                let other_range = other.get(category);
                self_range.end <= other_range.start || other_range.end <= self_range.start
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_size() {
        let set = PartSet {
            x: 1..4001,
            m: 1..4001,
            a: 1..4001,
            s: 1..4001,
        };

        assert_eq!(set.size(), 256_000_000_000_000);

        let set = PartSet {
            x: 1..4001,
            m: 1..1,
            a: 1..4001,
            s: 1..4001,
        };

        assert_eq!(set.size(), 0);
    }

    #[test]
    fn test_split() {
        let set = PartSet {
            x: 1..4001,
            m: 1000..2000,
            a: 1..10,
            s: 1..4001,
        };

        let (match_set, unmatch_set) = set.split(&Category::X, &Operator::GreaterThan, 3333);
        assert_eq!(
            match_set,
            Some(PartSet {
                x: 3334..4001,
                m: 1000..2000,
                a: 1..10,
                s: 1..4001,
            })
        );
        assert_eq!(
            unmatch_set,
            Some(PartSet {
                x: 1..3334,
                m: 1000..2000,
                a: 1..10,
                s: 1..4001,
            })
        );

        let (match_set, unmatch_set) = set.split(&Category::M, &Operator::GreaterThan, 3000);
        assert_eq!(match_set, None);
        assert_eq!(
            unmatch_set,
            Some(PartSet {
                x: 1..4001,
                m: 1000..2000,
                a: 1..10,
                s: 1..4001,
            })
        );

        let (match_set, unmatch_set) = set.split(&Category::A, &Operator::LessThan, 10);
        assert_eq!(
            match_set,
            Some(PartSet {
                x: 1..4001,
                m: 1000..2000,
                a: 1..10,
                s: 1..4001,
            })
        );
        assert_eq!(unmatch_set, None);

        let (match_set, unmatch_set) = set.split(&Category::A, &Operator::LessThan, 9);
        assert_eq!(
            match_set,
            Some(PartSet {
                x: 1..4001,
                m: 1000..2000,
                a: 1..9,
                s: 1..4001,
            })
        );
        assert_eq!(
            unmatch_set,
            Some(PartSet {
                x: 1..4001,
                m: 1000..2000,
                a: 9..10,
                s: 1..4001,
            })
        );
    }

    #[test]
    fn test_refine_already_disjoint() {
        let set1 = PartSet {
            x: 1..3,
            m: 1..3,
            a: 1..3,
            s: 1..3,
        };
        let set2 = PartSet {
            x: 3..5,
            m: 3..5,
            a: 3..5,
            s: 3..5,
        };

        assert_eq!(set1.refine(&set2), vec![set2]);
    }

    #[test]
    fn test_refine_not_disjoint() {
        let set1 = PartSet {
            x: 1..3,
            m: 1..3,
            a: 1..3,
            s: 1..3,
        };
        let set2 = PartSet {
            x: 2..4,
            m: 2..4,
            a: 2..4,
            s: 2..4,
        };

        let refinement = set1.refine(&set2);
        // both input sets have size 16, but they overlap on a 1x1x1x1 cube
        assert_eq!(refinement.iter().map(|set| set.size()).sum::<u64>(), 15);

        let set1 = PartSet {
            x: 2..3,
            m: 2..3,
            a: 1..4,
            s: 1..2,
        };
        let set2 = PartSet {
            x: 1..4,
            m: 1..4,
            a: 2..3,
            s: 1..2,
        };

        let refinement = set1.refine(&set2);

        // the two sets overlap on a 1x1x1x1 cube
        assert_eq!(
            refinement.iter().map(|set| set.size()).sum::<u64>(),
            set2.size() - 1
        );
    }

    #[test]
    fn test_refine_proper_subset() {
        let set1 = PartSet {
            x: 1..5,
            m: 1..5,
            a: 1..5,
            s: 1..5,
        };
        let set2 = PartSet {
            x: 2..4,
            m: 2..4,
            a: 2..4,
            s: 2..4,
        };

        let refinement = set1.refine(&set2);

        assert!(refinement.is_empty());

        let set1 = PartSet {
            x: 1..4000,
            m: 1..4000,
            a: 1..2006,
            s: 1..4000,
        };

        let refinement = set1.refine(&set1);

        assert!(refinement.is_empty());
    }
}
