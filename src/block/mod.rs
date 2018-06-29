use color::RED;
use piston_window::rectangle;
use piston_window::Context;
use piston_window::G2d;

const SIZE: f64 = 10.0;

pub struct Block {
    pub x: f64,
    pub y: f64,
    pub size: f64,
    pub direction: Direction,
    pub color: [f32; 4]
}

impl Block {
    pub fn new() -> Self {
        Block::from((0.0, 0.0))
    }

    pub fn render(&self, context: Context, graphics: &mut G2d) {
        rectangle(
            self.color, // red
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

    pub fn collides_with(&self, other: &Block) -> bool {
        self.x == other.x && self.y == other.y
        // let right_collides_with_left = (self.x + self.size) >= other.x;
        // let left_collides_with_right = self.x <= (other.x + other.size);
        // let top_collides_with_bottom = self.y <= (other.y + other.size);
        // let bottom_collides_with_top = (self.y + self.size) >= other.y;

        // !(right_collides_with_left || left_collides_with_right || top_collides_with_bottom
        //     || bottom_collides_with_top)
    }
}

impl From<(f64, f64)> for Block {
    fn from(coords: (f64, f64)) -> Self {
        Block {
            x: coords.0,
            y: coords.1,
            size: SIZE,
            direction: Direction::NONE,
            color: RED,
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


