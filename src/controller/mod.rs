use block::Direction;
use piston::input::Key;
use snake::Snake;

pub struct Controller<'a> {
    snake: &'a mut Snake,
}

impl<'a> Controller<'a> {
    pub fn new(snake: &'a mut Snake) -> Self {
        Controller {
            snake: snake
        }
    }

    pub fn process(&mut self, key: Key) {
        let new_direction = Controller::key_binding(key);
        if new_direction != self.snake.head.direction.opposite() {
            self.snake.head.direction = new_direction;
        }
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
}
