pub mod puzzle13a;
pub mod puzzle13b;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Element {
    Ash,
    Rock,
}

impl From<char> for Element {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Ash,
            '#' => Self::Rock,
            _ => panic!("unrecognized character"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Pattern(Vec<Vec<Element>>);

impl From<&str> for Pattern {
    fn from(input: &str) -> Self {
        Self(
            input
                .lines()
                .map(|line| line.chars().map(Element::from).collect())
                .collect(),
        )
    }
}

impl Pattern {
    fn height(&self) -> usize {
        self.0.len()
    }

    fn width(&self) -> usize {
        self.0[0].len()
    }

    fn get(&self, row_idx: usize, col_idx: usize) -> &Element {
        &self.0[row_idx][col_idx]
    }

    /// Checks if a pattern is symmetric across the _vertical_ line
    /// at a given position.
    fn is_symmetric_across_vert(&self, axis: usize) -> bool {
        // check corresponding reflected columns starting at
        // the axis of symmetry. stop once we reach one of the
        // edges of the pattern
        let mut offset = 0;

        while offset <= axis && axis + offset + 1 < self.width() {
            let left_col_idx = axis - offset;
            let right_col_idx = axis + offset + 1;

            if !(0..self.height())
                .all(|row_idx| self.get(row_idx, right_col_idx) == self.get(row_idx, left_col_idx))
            {
                return false;
            }

            offset += 1
        }

        true
    }

    fn is_symmetric_across_horiz(&self, axis: usize) -> bool {
        // check corresponding reflected rows starting at
        // the axis of symmetry. stop once we reach the top or bottom
        // edges of the pattern
        let mut offset = 0;

        while offset <= axis && axis + offset + 1 < self.height() {
            let left_row_idx = axis - offset;
            let right_row_idx = axis + offset + 1;

            if !(0..self.width())
                .all(|col_idx| self.get(right_row_idx, col_idx) == self.get(left_row_idx, col_idx))
            {
                return false;
            }

            offset += 1
        }

        true
    }
}
