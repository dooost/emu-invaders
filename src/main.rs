mod display;
mod invaders;
mod io;

use invaders::Invaders;

fn main() {
    let invaders = Invaders::new();

    invaders.run();
}
