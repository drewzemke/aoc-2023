pub mod puzzle10a;
pub mod puzzle10b;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum PipeTile {
    Vertical,
    Horizontal,
    TopRight,
    TopLeft,
    BottomRight,
    BottomLeft,
    Start,
    Nothing,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq)]
pub struct PipeGrid(Vec<Vec<PipeTile>>);

type GridPoint = (usize, usize);

impl PipeGrid {
    pub fn start(&self) -> (GridPoint, Direction) {
        let start = self
            .0
            .iter()
            .enumerate()
            .find_map(|(row_idx, row)| {
                row.iter()
                    .position(|pipe| pipe == &PipeTile::Start)
                    .map(|col_idx| (row_idx, col_idx))
            })
            .unwrap();

        // probe the grid to find a valid starting direction
        // can we go left?
        if start.0 > 0 {
            let left_neighbor = self.get((start.0, start.1 - 1));
            if let PipeTile::Horizontal | PipeTile::TopRight | PipeTile::BottomRight = left_neighbor
            {
                return (start, Direction::Left);
            }
        }
        // can we go up?
        if start.1 > 0 {
            let above_neighbor = self.get((start.0 - 1, start.1));
            if let PipeTile::Vertical | PipeTile::BottomRight | PipeTile::BottomLeft =
                above_neighbor
            {
                return (start, Direction::Up);
            }
        }
        // okay well we can definitely go right then (two of the four directions must be valid)
        (start, Direction::Right)
    }

    fn get(&self, point: GridPoint) -> &PipeTile {
        &self.0[point.0][point.1]
    }

    fn find_loop(&mut self) -> Vec<GridPoint> {
        // find the start location
        let (start, start_direction) = self.start();

        let mut points = vec![];
        let mut current = start;
        let mut direction = start_direction;
        let mut end_direction = start_direction;

        while points.is_empty() || current != start {
            // walk in the direction
            current = match direction {
                Direction::Up => (current.0 - 1, current.1),
                Direction::Down => (current.0 + 1, current.1),
                Direction::Left => (current.0, current.1 - 1),
                Direction::Right => (current.0, current.1 + 1),
            };

            // add the point to the list
            points.push(current);

            // look at the tile to figure out a new direction
            let tile = self.get(current);
            direction = match (direction, tile) {
                (Direction::Up, PipeTile::Vertical) => Direction::Up,
                (Direction::Up, PipeTile::BottomRight) => Direction::Right,
                (Direction::Up, PipeTile::BottomLeft) => Direction::Left,

                (Direction::Down, PipeTile::Vertical) => Direction::Down,
                (Direction::Down, PipeTile::TopRight) => Direction::Right,
                (Direction::Down, PipeTile::TopLeft) => Direction::Left,

                (Direction::Left, PipeTile::Horizontal) => Direction::Left,
                (Direction::Left, PipeTile::TopRight) => Direction::Up,
                (Direction::Left, PipeTile::BottomRight) => Direction::Down,

                (Direction::Right, PipeTile::Horizontal) => Direction::Right,
                (Direction::Right, PipeTile::TopLeft) => Direction::Up,
                (Direction::Right, PipeTile::BottomLeft) => Direction::Down,

                (dir, PipeTile::Start) => {
                    end_direction = dir;
                    start_direction
                }
                _ => panic!("could not navigate {direction:?} when current tile was {tile:?}"),
            };
        }

        // update the start tile
        let start_tile = match (start_direction, end_direction) {
            (Direction::Up, Direction::Up) => PipeTile::Vertical,
            (Direction::Up, Direction::Left) => PipeTile::TopRight,
            (Direction::Up, Direction::Right) => PipeTile::TopLeft,

            (Direction::Down, Direction::Down) => PipeTile::Vertical,
            (Direction::Down, Direction::Left) => PipeTile::BottomRight,
            (Direction::Down, Direction::Right) => PipeTile::BottomLeft,

            (Direction::Left, Direction::Up) => PipeTile::BottomLeft,
            (Direction::Left, Direction::Down) => PipeTile::TopLeft,
            (Direction::Left, Direction::Left) => PipeTile::Horizontal,

            (Direction::Right, Direction::Up) => PipeTile::BottomRight,
            (Direction::Right, Direction::Down) => PipeTile::TopRight,
            (Direction::Right, Direction::Right) => PipeTile::Horizontal,

            _ => panic!("invalid state while computing start tile"),
        };
        self.0[start.0][start.1] = start_tile;

        points
    }

    pub fn count_in_loop(&self, pipe_loop: &[(usize, usize)]) -> u32 {
        let mut count = 0;

        self.0.iter().enumerate().for_each(|(row_idx, row)| {
            let mut count_status = State::Not;

            row.iter().enumerate().for_each(|(col_idx, tile)| {
                if pipe_loop.contains(&(row_idx, col_idx)) {
                    // we're scanning left to right, so every time
                    // we hit a tile in the loop, we either start/stop counting (if it's vertical)
                    // or start/stop considering counting if it's TopRight or BottomRight
                    count_status.process_tile(tile);
                } else {
                    // if we're not looking at a tile in the loop, potentially increment the count
                    if count_status == State::Counting {
                        count += 1;
                    }
                }
            });
        });
        count
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum State {
    Counting,
    Not,
    Almost(PipeTile),
    AlmostNot(PipeTile),
}

impl State {
    fn process_tile(&mut self, pipe_tile: &PipeTile) {
        *self = match (pipe_tile, &self) {
            (PipeTile::Vertical, State::Counting) => State::Not,
            (PipeTile::Vertical, State::Not) => State::Counting,

            (PipeTile::TopRight, State::Counting) => State::AlmostNot(PipeTile::TopRight),
            (PipeTile::TopRight, State::Not) => State::Almost(PipeTile::TopRight),

            (PipeTile::BottomRight, State::Counting) => State::AlmostNot(PipeTile::BottomRight),
            (PipeTile::BottomRight, State::Not) => State::Almost(PipeTile::BottomRight),

            (PipeTile::BottomLeft, State::Almost(PipeTile::BottomRight)) => State::Not,
            (PipeTile::TopLeft, State::Almost(PipeTile::BottomRight)) => State::Counting,

            (PipeTile::BottomLeft, State::Almost(PipeTile::TopRight)) => State::Counting,
            (PipeTile::TopLeft, State::Almost(PipeTile::TopRight)) => State::Not,

            (PipeTile::BottomLeft, State::AlmostNot(PipeTile::BottomRight)) => State::Counting,
            (PipeTile::TopLeft, State::AlmostNot(PipeTile::BottomRight)) => State::Not,

            (PipeTile::BottomLeft, State::AlmostNot(PipeTile::TopRight)) => State::Not,
            (PipeTile::TopLeft, State::AlmostNot(PipeTile::TopRight)) => State::Counting,

            (PipeTile::Horizontal, State::Almost(title)) => State::Almost(title.clone()),
            (PipeTile::Horizontal, State::AlmostNot(title)) => State::AlmostNot(title.clone()),

            _ => panic!("invalid state ({pipe_tile:?}, {self:?}) in tile processing"),
        };
    }
}

impl From<&str> for PipeGrid {
    fn from(input: &str) -> Self {
        let pipes = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '|' => PipeTile::Vertical,
                        '-' => PipeTile::Horizontal,
                        'L' => PipeTile::TopRight,
                        'J' => PipeTile::TopLeft,
                        '7' => PipeTile::BottomLeft,
                        'F' => PipeTile::BottomRight,
                        'S' => PipeTile::Start,
                        '.' => PipeTile::Nothing,
                        _ => panic!("unrecognized character"),
                    })
                    .collect()
            })
            .collect();
        Self(pipes)
    }
}
