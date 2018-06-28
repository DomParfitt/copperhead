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
        let mut rng = thread_rng();
        let x = rng.gen_range(0.0, 500.0);
        let y = rng.gen_range(0.0, 400.0);
        Food {
            is_consumed: false,
            block: Block::from((x, y))
        }
    }

    pub fn render(&self, context: Context, graphics: &mut G2d) {
        self.block.render(context, graphics);
    }
}