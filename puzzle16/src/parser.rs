use crate::{EnergizedTile, Tile, TileGrid};

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Nothing,
            '|' => Self::SplitterVertical,
            '-' => Self::SplitterHorizontal,
            '/' => Self::DiagonalUp,
            '\\' => Self::DiagonalDown,
            _ => panic!("unrecognized character"),
        }
    }
}

impl From<&str> for TileGrid {
    fn from(input: &str) -> Self {
        let tiles = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|char| EnergizedTile {
                        tile: char.into(),
                        beams: vec![],
                    })
                    .collect()
            })
            .collect();
        Self(tiles)
    }
}
