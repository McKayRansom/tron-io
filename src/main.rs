use game::Game;
use macroquad::window::next_frame;


mod snake;
mod game;


#[macroquad::main("Tron-IO")]
async fn main() {

    let mut game = Game::new();

    loop {
        if game.update() {
            game = Game::new();
        }
        next_frame().await;
    }
}
