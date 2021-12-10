mod display;
mod invaders;

use invaders::Invaders;

fn main() {
    let invaders = Invaders::new();

    invaders.run();
}
