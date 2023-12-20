pub mod parser;
pub mod puzzle16a;
pub mod puzzle16b;

#[derive(Debug, PartialEq, Eq)]
pub enum Tile {
    Nothing,
    SplitterVertical,
    SplitterHorizontal,
    DiagonalUp,
    DiagonalDown,
}

impl Tile {
    /// Gets a list of the directions of the reflected beams of light that enter
    /// this tile heading in the given direction
    fn map_direction(&self, direction: Direction) -> Vec<Direction> {
        match (self, direction) {
            (Tile::Nothing, dir) => vec![dir],

            (Tile::SplitterVertical, Direction::Left | Direction::Right) => {
                vec![Direction::Up, Direction::Down]
            }
            (Tile::SplitterVertical, dir) => vec![dir],

            (Tile::SplitterHorizontal, Direction::Up | Direction::Down) => {
                vec![Direction::Left, Direction::Right]
            }
            (Tile::SplitterHorizontal, dir) => vec![dir],

            (Tile::DiagonalUp, Direction::Up) => vec![Direction::Right],
            (Tile::DiagonalUp, Direction::Down) => vec![Direction::Left],
            (Tile::DiagonalUp, Direction::Left) => vec![Direction::Down],
            (Tile::DiagonalUp, Direction::Right) => vec![Direction::Up],

            (Tile::DiagonalDown, Direction::Up) => vec![Direction::Left],
            (Tile::DiagonalDown, Direction::Down) => vec![Direction::Right],
            (Tile::DiagonalDown, Direction::Left) => vec![Direction::Up],
            (Tile::DiagonalDown, Direction::Right) => vec![Direction::Down],
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    /// Gets the grid location that results from taking one step in this direction
    /// starting from the given location.
    fn map_location(&self, location: (usize, usize)) -> Option<(usize, usize)> {
        match self {
            Direction::Up => {
                if location.0 == 0 {
                    return None;
                }
                Some((location.0 - 1, location.1))
            }
            Direction::Down => Some((location.0 + 1, location.1)),
            Direction::Left => {
                if location.1 == 0 {
                    return None;
                }
                Some((location.0, location.1 - 1))
            }
            Direction::Right => Some((location.0, location.1 + 1)),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct EnergizedTile {
    /// The grid object at this location.
    tile: Tile,

    /// Records the directions with which beams have
    /// _entered_ this tile.
    beams: Vec<Direction>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct TileGrid(Vec<Vec<EnergizedTile>>);

impl TileGrid {
    fn get_mut(&mut self, (row_idx, col_idx): (usize, usize)) -> Option<&mut EnergizedTile> {
        self.0.get_mut(row_idx)?.get_mut(col_idx)
    }

    pub fn trace_beams(&mut self, location: (usize, usize), direction: Direction) {
        // if there's no tile here, there's nothing to do
        let Some(EnergizedTile { tile, beams }) = self.get_mut(location) else {
            return;
        };

        // have we been here before with this direction? if so, we've already traced
        // out this path, so there's no more work to do.
        // otherwise, record the current direction in the record
        if beams.contains(&direction) {
            return;
        } else {
            beams.push(direction.clone());
        }

        // based on the current tile and the direciton of the beam , determine where
        // the beam should go from here and with what direction(s)
        let next_directions = tile.map_direction(direction);

        for next_direction in next_directions {
            let Some(next_location) = next_direction.map_location(location) else {
                continue;
            };
            self.trace_beams(next_location, next_direction);
        }
    }

    pub fn energized_tiles(&self) -> usize {
        self.0
            .iter()
            .map(|row| {
                row.iter()
                    .map(|EnergizedTile { beams, .. }| if beams.is_empty() { 0 } else { 1 })
                    .sum::<usize>()
            })
            .sum::<usize>()
    }
}
