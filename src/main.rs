extern crate piston_window;
extern crate piston;

use piston_window::*;

mod block;
mod controller;
mod snake;
mod traits;

use snake::*;

const WINDOW_SIZE: [u32; 2] = [500, 400];
const COUNT_DOWN: u32 = 60;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Snake!", WINDOW_SIZE).build().unwrap();

    let mut snake = Snake::new();
    // let mut controller = Controller::new(&mut snake);

    let mut counter = COUNT_DOWN;
    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics| {
            clear([1.0; 4], graphics);
            snake.render(context, graphics);            
        });

        if let Some(Button::Keyboard(key)) = event.press_args() {
            snake.process(key)
        }

        counter -= 1;
        if counter <= 0 {
            snake.update();
            counter = COUNT_DOWN;
        }
    }
}
