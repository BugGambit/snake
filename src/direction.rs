use crate::point::Point;
use rand::Rng;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

impl Direction {
    pub fn rand() -> Self {
        let dirs = [
            Direction::NORTH,
            Direction::EAST,
            Direction::SOUTH,
            Direction::WEST,
        ];
        let mut rng = rand::thread_rng();
        dirs[rng.gen_range(0, 4)]
    }

    pub fn to_delta_point(&self) -> Point {
        match &self {
            Direction::NORTH => Point { x: 0, y: -1 },
            Direction::SOUTH => Point { x: 0, y: 1 },
            Direction::EAST => Point { x: 1, y: 0 },
            Direction::WEST => Point { x: -1, y: 0 },
        }
    }

    pub fn negate(&self) -> Direction {
        match &self {
            Direction::NORTH => Direction::SOUTH,
            Direction::SOUTH => Direction::NORTH,
            Direction::EAST => Direction::WEST,
            Direction::WEST => Direction::EAST,
        }
    }
}
