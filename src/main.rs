mod invaders;
mod io;
mod view;

use invaders::Invaders;

fn main() {
    let invaders = Invaders::new();

    invaders.run();
}
