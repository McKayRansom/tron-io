use context::Context;
use game::Game;
use macroquad::window::{next_frame, Conf};


// mod bike;
mod game;
// mod grid;
mod context;


fn window_conf() -> Conf {
    Conf {
        fullscreen: false,
        // high-dpi seems to change the zoom on webassembly??
        high_dpi: true,
        // icon: Some(Icon {
        //     small: include_bytes!("../icons/16x16.rgba").to_owned(),
        //     medium: include_bytes!("../icons/32x32.rgba").to_owned(),
        //     big: include_bytes!("../icons/64x64.rgba").to_owned(),
        // }),
        // platform: miniquad::conf::Platform {
        //     linux_backend: miniquad::conf::LinuxBackend::WaylandWithX11Fallback,
        //     ..Default::default()
        // },
        window_height: 720,
        window_resizable: true,
        window_title: String::from("Tron-IO"),
        window_width: 1280,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {

    let context = Context::default().await;

    let mut game = Game::new();

    let mut won = 0;
    let mut lost = 0;

    loop {
        if game.update(won, lost, &context) {
            if game.game_won {
                won += 1;
            } else {
                lost += 1;
            }
            game = Game::new();
        }
        next_frame().await;
    }
}
