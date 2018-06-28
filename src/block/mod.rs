use piston_window::rectangle;
use piston_window::G2d;
use piston_window::Context;

const SIZE: f64 = 10.0;

pub struct Block {
    pub x: f64,
    pub y: f64,
    pub size: f64,
    pub direction: Direction,
}

impl Block {
    pub fn new() -> Self {
        Block::from((0.0, 0.0))
    }

    pub fn render(&self, context: Context, graphics: &mut G2d) {
        rectangle(
            [1.0, 0.0, 0.0, 1.0], // red
            [self.x, self.y, self.size, self.size],
            context.transform,
            graphics,
        );
    }

    pub fn update(&mut self) {
        match self.direction {
                Direction::UP => {
                    self.y -= self.size;
                }
                Direction::DOWN => {
                    self.y += self.size;
                }
                Direction::LEFT => {
                    self.x -= self.size;
                }
                Direction::RIGHT => {
                    self.x += self.size;
                }
                _ => {}
            }
    }
}

impl From<(f64, f64)> for Block {
    fn from(coords: (f64, f64)) -> Self {
        Block {
            x: coords.0,
            y: coords.1,
            size: SIZE,
            direction: Direction::NONE,
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

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
    NONE,
}

impl Direction {
    pub fn opposite(&self) -> Self {
        match self {
            Direction::UP => Direction::DOWN,
            Direction::DOWN => Direction::UP,
            Direction::LEFT => Direction::RIGHT,
            Direction::RIGHT => Direction::LEFT,
            Direction::NONE => Direction::NONE,
        }
    }
}
