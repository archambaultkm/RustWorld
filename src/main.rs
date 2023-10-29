mod game_specs;
mod core;
mod creation;
mod rendering;

use crate::core::game::Game;

fn main() {
    Game::run(&Game::new());
}
