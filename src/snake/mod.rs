use piston_window::rectangle;
use block::*;
use piston_window::Context;
use piston_window::G2d;

const INITIAL: (f64, f64, Direction) = (0.0, 0.0, Direction::RIGHT);

pub struct Snake {
    pub body: Vec<Block>,
}

impl Snake {
    pub fn new() -> Self {
        Snake {
            body: vec![Block::from(INITIAL)],
        }
    }

    pub fn render(&self, context: Context, graphics: &mut G2d) {
        for block in self.body.iter() {
            rectangle(
                [1.0, 0.0, 0.0, 1.0], // red
                [block.x, block.y, block.size, block.size],
                context.transform,
                graphics,
            );
        }
    }

    pub fn update(&mut self) {
        // let previous_direction
        for block in self.body.iter_mut() {
            match block.direction {
                UP => {
                    block.y -= block.size;
                }
                DOWN => {
                    block.y += block.size;
                }
                LEFT => {
                    block.x -= block.size;
                }
                RIGHT => {
                    block.x += block.size;
                }
                _ => {}
            }
        }
    }

    fn add_block(&mut self) {
        let block = self.new_block();
        self.body.push(block);
    }

    fn new_block(&self) -> Block {
        let last: &Block = &self.body[self.body.len() - 1];
        let (mut x, mut y) = (last.x, last.y);
        match last.direction {
            UP => {
                y += last.size;
            }
            DOWN => {
                y -= last.size;
            }
            LEFT => {
                x += last.size;
            }
            RIGHT => {
                x -= last.size;
            }
            _ => {}
        }

        Block::from((x, y, last.direction))
    }

}
