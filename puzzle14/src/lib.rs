pub mod parser;
pub mod puzzle14a;
pub mod puzzle14b;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
enum Element {
    RollingRock,
    StationaryRock,
    Nothing,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Platform(PlatformRep);

#[derive(Debug, PartialEq, Eq, Clone)]
enum PlatformRep {
    Rows(PlatformRows),
    Columns(PlatformColumns),
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct PlatformRows(Vec<Vec<Element>>);

#[derive(Debug, PartialEq, Eq, Clone)]
struct PlatformColumns(Vec<Vec<Element>>);

fn transpose<T: Clone>(matrix: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    (0..matrix[0].len())
        .map(|outer_idx| {
            (0..matrix.len())
                .map(|inner_idx| matrix[inner_idx][outer_idx].clone())
                .collect()
        })
        .collect()
}

impl From<&PlatformRows> for PlatformColumns {
    fn from(rows: &PlatformRows) -> Self {
        Self(transpose(&rows.0))
    }
}

impl From<&PlatformColumns> for PlatformRows {
    fn from(cols: &PlatformColumns) -> Self {
        Self(transpose(&cols.0))
    }
}

pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Platform {
    fn swap_rep(&mut self) {
        self.0 = match self.0 {
            PlatformRep::Rows(ref rows) => PlatformRep::Columns(rows.into()),
            PlatformRep::Columns(ref cols) => PlatformRep::Rows(cols.into()),
        }
    }

    pub fn north_load(&self) -> usize {
        let rows: PlatformRows = match self.0 {
            PlatformRep::Rows(ref rows) => rows.clone(),
            PlatformRep::Columns(ref cols) => cols.into(),
        };

        // `RollingRock`s have weight equal to their distance to the bottom row (plus one)
        let height = rows.0.len();
        rows.0
            .iter()
            .enumerate()
            .map(|(row_idx, row)| {
                (height - row_idx)
                    * row
                        .iter()
                        .filter(|element| **element == Element::RollingRock)
                        .count()
            })
            .sum::<usize>()
    }

    pub fn tilt(&mut self, direction: Direction) {
        // use the representation most amenable to tilting in the given direction
        match &mut self.0 {
            PlatformRep::Columns(cols) => match direction {
                Direction::North => cols.tilt_north(),
                Direction::South => cols.tilt_south(),
                direction => {
                    self.swap_rep();
                    self.tilt(direction);
                }
            },
            PlatformRep::Rows(rows) => match direction {
                Direction::East => rows.tilt_east(),
                Direction::West => rows.tilt_west(),
                direction => {
                    self.swap_rep();
                    self.tilt(direction);
                }
            },
        };
    }
}

impl PlatformColumns {
    pub fn tilt_north(&mut self) {
        // operate on each column individually
        // break it up into chunks based on the stationary rocks in it
        // within each chunk, move all of the rolling rocks to the front
        for col in &mut self.0 {
            for chunk in col.split_mut(|element| *element == Element::StationaryRock) {
                // the derived order on `Element` places `RollingRock` before `Nothing`
                chunk.sort();
            }
        }
    }

    pub fn tilt_south(&mut self) {
        // same as above, but opposite
        for col in &mut self.0 {
            for chunk in col.split_mut(|element| *element == Element::StationaryRock) {
                // the derived order on `Element` places `RollingRock` before `Nothing`, so
                // do the opposite of that
                chunk.sort_by(|el1, el2| el2.cmp(el1));
            }
        }
    }
}

impl PlatformRows {
    pub fn tilt_west(&mut self) {
        // same as tilt_north, but moving horizontally
        for row in &mut self.0 {
            for chunk in row.split_mut(|element| *element == Element::StationaryRock) {
                chunk.sort();
            }
        }
    }

    pub fn tilt_east(&mut self) {
        // yeah
        for row in &mut self.0 {
            for chunk in row.split_mut(|element| *element == Element::StationaryRock) {
                chunk.sort_by(|el1, el2| el2.cmp(el1));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rows_to_cols() {
        let rows = PlatformRows(vec![
            vec![
                Element::RollingRock,
                Element::Nothing,
                Element::StationaryRock,
            ],
            vec![
                Element::StationaryRock,
                Element::Nothing,
                Element::RollingRock,
            ],
        ]);

        let cols = PlatformColumns::from(&rows);
        assert_eq!(
            cols,
            PlatformColumns(vec![
                vec![Element::RollingRock, Element::StationaryRock],
                vec![Element::Nothing, Element::Nothing],
                vec![Element::StationaryRock, Element::RollingRock],
            ])
        )
    }

    #[test]
    fn test_cols_to_rows() {
        let cols = PlatformColumns(vec![
            vec![Element::RollingRock, Element::StationaryRock],
            vec![Element::Nothing, Element::Nothing],
            vec![Element::StationaryRock, Element::RollingRock],
        ]);
        let rows = PlatformRows::from(&cols);

        assert_eq!(
            rows,
            PlatformRows(vec![
                vec![
                    Element::RollingRock,
                    Element::Nothing,
                    Element::StationaryRock
                ],
                vec![
                    Element::StationaryRock,
                    Element::Nothing,
                    Element::RollingRock
                ],
            ])
        )
    }

    #[test]
    fn test_tilt_north() {
        let mut platform = Platform(PlatformRep::Columns(PlatformColumns(vec![
            vec![Element::RollingRock, Element::Nothing, Element::RollingRock],
            vec![
                Element::Nothing,
                Element::RollingRock,
                Element::StationaryRock,
            ],
            vec![
                Element::StationaryRock,
                Element::RollingRock,
                Element::RollingRock,
            ],
        ])));

        platform.tilt(Direction::North);

        assert_eq!(
            platform,
            Platform(PlatformRep::Columns(PlatformColumns(vec![
                vec![Element::RollingRock, Element::RollingRock, Element::Nothing],
                vec![
                    Element::RollingRock,
                    Element::Nothing,
                    Element::StationaryRock,
                ],
                vec![
                    Element::StationaryRock,
                    Element::RollingRock,
                    Element::RollingRock,
                ],
            ])))
        );
    }

    #[test]
    fn test_compute_load() {
        let platform = Platform(PlatformRep::Rows(PlatformRows(vec![
            vec![Element::RollingRock, Element::Nothing, Element::RollingRock],
            vec![
                Element::StationaryRock,
                Element::Nothing,
                Element::RollingRock,
            ],
            vec![
                Element::StationaryRock,
                Element::Nothing,
                Element::RollingRock,
            ],
        ])));
        assert_eq!(platform.north_load(), 9);
    }
}
