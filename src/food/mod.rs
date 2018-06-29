use piston_window::G2d;
use piston_window::Context;
use rand::thread_rng;
use block::Block;
use rand::Rng;

pub struct Food {
    pub is_consumed: bool,
    pub block: Block
}

impl Food {
    pub fn new() -> Self {
        let mut food = Food {
            is_consumed: false,
            block: Block::new()
        };

        let mut x = Food::get_random_coord(food.block.size);
        let mut y = Food::get_random_coord(food.block.size);
        println!("Placing food at ({}, {})", x, y);
        food.block = Block::from((x, y));
        food
    }

    pub fn render(&self, context: Context, graphics: &mut G2d) {
        self.block.render(context, graphics);
    }

    fn get_random_coord(block_size: f64) -> f64 {
        let mut rng = thread_rng();
        let mut coord = rng.gen_range(0, 500) as f64;
        while coord % block_size != 0.0 {
            coord = rng.gen_range(0, 500) as f64;
        }

        coord
    }
}