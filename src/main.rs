mod shader;
mod camera;
mod game_window;
mod cube;
mod texture;
mod game_specs;
mod world;
mod renderer;
mod game;
mod chunk;
mod block_config;

use crate::game::Game;

fn main() {
    let game = Game::new();
    game.run();
}
