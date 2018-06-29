extern crate piston;
extern crate piston_window;
extern crate rand;

use piston_window::*;

mod block;
mod color;
mod controller;
mod food;
mod snake;
mod traits;

use color::*;
use food::Food;
use snake::Snake;

const WINDOW_SIZE: [u32; 2] = [500, 400];
const COUNT_DOWN: u32 = 30;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Snake!", WINDOW_SIZE).build().unwrap();

    let mut snake = Snake::new();
    let mut food = Food::new();

    let mut counter = COUNT_DOWN;
    let mut game_over = false;
    while let Some(event) = window.next() {
        if !game_over {
            window.draw_2d(&event, |context, graphics| {
                clear(WHITE, graphics);
                snake.render(context, graphics);
                food.render(context, graphics);
            });

            if let Some(Button::Keyboard(key)) = event.press_args() {
                snake.process(key)
            }

            counter -= 1;
            if counter <= 0 {
                snake.update();
                counter = COUNT_DOWN;
            }

            if check_collision_with_food(&mut snake, &food) {
                food = Food::new();
            }

            game_over = check_collision(&mut snake);
        } else {
            window.draw_2d(&event, |context, graphics| {
                clear(BLACK, graphics);
                snake.render(context, graphics);
                food.render(context, graphics);
            });
        }
    }
}

fn check_collision(snake: &mut Snake) -> bool {
    if check_collision_with_wall(snake) {
        snake.add_block();
        true
    } else {
        check_collision_with_self(snake)
    }
}

fn check_collision_with_wall(snake: &Snake) -> bool {
    let x = snake.head.x >= 0.0 && snake.head.x <= WINDOW_SIZE[0].into();
    let y = snake.head.y >= 0.0 && snake.head.y <= WINDOW_SIZE[1].into();
    !(x && y)
    // false
}

fn check_collision_with_self(snake: &Snake) -> bool {
    for block in snake.body.iter() {
        if snake.head.collides_with(block) {
            return true;
        }
    }
    false
}

fn check_collision_with_food(snake: &mut Snake, food: &Food) -> bool {
    if snake.head.collides_with(&food.block) {
        snake.add_block();
        return true;
    }

    false
}
