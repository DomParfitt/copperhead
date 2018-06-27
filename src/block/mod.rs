const SIZE: f64 = 10.0;

pub struct Block {
    pub x: f64,
    pub y: f64,
    pub size: f64,
    pub direction: Direction
}

impl Block {
    pub fn new() -> Self {
        Block::from((0.0, 0.0))
    }
}

impl From<(f64, f64)> for Block {
    fn from(coords: (f64, f64)) -> Self {
        Block {
            x: coords.0,
            y: coords.1,
            size: SIZE,
            direction: Direction::NONE
        }
    }
}

impl From<(f64, f64, Direction)> for Block {
    fn from(details: (f64, f64, Direction)) -> Self {
        let mut block = Block::from((details.0, details.1));
        block.direction = details.2;
        block
    }
}

#[derive(Copy, Clone)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
    NONE
}