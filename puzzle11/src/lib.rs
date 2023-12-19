pub mod puzzle11a;
pub mod puzzle11b;

#[derive(Debug)]
pub struct Universe {
    galaxies: Vec<Galaxy>,
    empty_rows: Vec<usize>,
    empty_cols: Vec<usize>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Galaxy(usize, usize);

impl From<&str> for Universe {
    fn from(input: &str) -> Self {
        let galaxies: Vec<Galaxy> = input
            .lines()
            .enumerate()
            .flat_map(|(row_idx, row)| {
                row.chars()
                    .enumerate()
                    .filter_map(|(col_idx, c)| {
                        if c == '#' {
                            Some(Galaxy(row_idx, col_idx))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<Galaxy>>()
            })
            .collect();

        let filled_rows: Vec<_> = galaxies.iter().map(|Galaxy(row_idx, _)| row_idx).collect();
        let filled_cols: Vec<_> = galaxies.iter().map(|Galaxy(_, col_idx)| col_idx).collect();

        let num_cols = input.lines().count();
        let num_rows = input.lines().next().unwrap().len();

        let empty_rows: Vec<_> = (0..num_cols)
            .filter(|row_idx| !filled_rows.contains(&row_idx))
            .collect();
        let empty_cols: Vec<_> = (0..num_rows)
            .filter(|col_idx| !filled_cols.contains(&col_idx))
            .collect();

        Self {
            galaxies,
            empty_rows,
            empty_cols,
        }
    }
}

impl Universe {
    /// Enumerates all (n choose 2) pairs of galaxies in a `Universe`.
    pub fn galaxy_pairs(&self) -> Vec<(&Galaxy, &Galaxy)> {
        self.galaxies[..]
            .iter()
            .enumerate()
            .flat_map(|(idx, galaxy1)| {
                self.galaxies[idx + 1..]
                    .iter()
                    .map(|galaxy2| (galaxy1, galaxy2))
                    .collect::<Vec<_>>()
            })
            .collect()
    }

    /// Computes the distance of the "shortest path" between two galaxies _after_
    /// all of the rows and columns without galaxies have expanded
    pub fn expanded_distance(
        &self,
        galaxy1: &Galaxy,
        galaxy2: &Galaxy,
        expand_factor: usize,
    ) -> usize {
        let Galaxy(row1, col1) = galaxy1;
        let Galaxy(row2, col2) = galaxy2;

        let row_range = if row1 < row2 { row1..row2 } else { row2..row1 };
        let col_range = if col1 < col2 { col1..col2 } else { col2..col1 };

        let horizontal_dist = col_range.end - col_range.start
            + (expand_factor - 1)
                * self
                    .empty_cols
                    .iter()
                    .filter(|col| col_range.contains(col))
                    .count();
        let vertical_dist = row_range.end - row_range.start
            + (expand_factor - 1)
                * self
                    .empty_rows
                    .iter()
                    .filter(|row| row_range.contains(row))
                    .count();

        horizontal_dist + vertical_dist
    }
}
