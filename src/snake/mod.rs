use block::*;
use piston::input::Key;
use piston_window::Context;
use piston_window::G2d;

const INITIAL: (f64, f64, Direction) = (50.0, 50.0, Direction::RIGHT);
const STARTING_SIZE: usize = 5;

pub struct Snake {
    pub head: Block,
    pub body: Vec<Block>,
}

impl Snake {
    pub fn new() -> Self {
        let mut snake = Snake {
            head: Block::from(INITIAL),
            body: vec![],
        };
        for _ in 1..STARTING_SIZE {
            snake.add_block();
        }
        snake
    }

    pub fn render(&self, context: Context, graphics: &mut G2d) {
        self.head.render(context, graphics);
        for block in self.body.iter() {
            block.render(context, graphics);
        }
    }

    pub fn update(&mut self) {
        self.head.update();
        let mut prior_direction = self.head.direction;
        for block in self.body.iter_mut() {
            block.update();
            let old_direction = block.direction;
            block.direction = prior_direction;
            prior_direction = old_direction;
        }
    }

    pub fn process(&mut self, key: Key) {
        let new_direction = Snake::key_binding(key);
        if new_direction != self.head.direction.opposite() {
            self.head.direction = new_direction;
        }
    }

    pub fn add_block(&mut self) {
        let block = self.new_block();
        self.body.push(block);
    }

    fn key_binding(key: Key) -> Direction {
        match key {
            Key::Up => Direction::UP,
            Key::Down => Direction::DOWN,
            Key::Left => Direction::LEFT,
            Key::Right => Direction::RIGHT,
            _ => Direction::NONE,
        }
    }

    fn new_block(&self) -> Block {
        let last: &Block = if self.body.is_empty() {
            &self.head
        } else {
            &self.body[self.body.len() - 1]
        };

        let (mut x, mut y) = (last.x, last.y);
        match last.direction {
            Direction::UP => {
                y += last.size;
            }
            Direction::DOWN => {
                y -= last.size;
            }
            Direction::LEFT => {
                x += last.size;
            }
            Direction::RIGHT => {
                x -= last.size;
            }
            _ => {}
        }

        Block::from((x, y, last.direction))
    }
}
