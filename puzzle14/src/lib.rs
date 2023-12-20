pub mod parser;
pub mod puzzle14a;
pub mod puzzle14b;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
enum Element {
    RollingRock,
    StationaryRock,
    Nothing,
}

#[derive(Debug, PartialEq, Eq)]
pub struct PlatformRows(Vec<Vec<Element>>);

#[derive(Debug, PartialEq, Eq)]
pub struct PlatformColumns(Vec<Vec<Element>>);

impl From<PlatformRows> for PlatformColumns {
    fn from(rows: PlatformRows) -> Self {
        Self(
            (0..rows.0[0].len())
                .map(|col_idx| {
                    (0..rows.0.len())
                        .map(|row_idx| rows.0[row_idx][col_idx].clone())
                        .collect()
                })
                .collect(),
        )
    }
}

impl From<PlatformColumns> for PlatformRows {
    fn from(rows: PlatformColumns) -> Self {
        Self(
            (0..rows.0[0].len())
                .map(|col_idx| {
                    (0..rows.0.len())
                        .map(|row_idx| rows.0[row_idx][col_idx].clone())
                        .collect()
                })
                .collect(),
        )
    }
}

impl PlatformRows {
    pub fn north_load(&self) -> usize {
        // `RollingRock`s have weight equal to their distance to the bottom row (plus one)
        let height = self.0.len();
        self.0
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rows_to_cols() {
        let rows = PlatformRows::from(
            r#"O.#
#.O"#,
        );

        let cols = PlatformColumns::from(rows);
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
        let rows = PlatformRows::from(cols);

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
        let mut platform = PlatformColumns(vec![
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
        ]);

        platform.tilt_north();

        assert_eq!(
            platform,
            PlatformColumns(vec![
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
            ])
        );
    }

    #[test]
    fn test_compute_load() {
        let platform = PlatformRows(vec![
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
        ]);
        assert_eq!(platform.north_load(), 9);
    }
}
