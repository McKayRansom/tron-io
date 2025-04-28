use context::Context;
use gameplay::Gameplay;
use macroquad::window::{Conf, next_frame};
use scene::{main_menu::MainMenu, EScene};

// mod bike;
mod gameplay;
// mod grid;
mod context;
mod scene;
mod text;
mod input;
mod audio;
mod assets_path;
mod ui;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const PKG_NAME: &str = env!("CARGO_PKG_NAME");

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
    let mut ctx = Context::default().await;

    let mut current_scene: Box<dyn scene::Scene> =
        Box::new(scene::main_menu::MainMenu::new(&mut ctx).await);

    loop {
        ctx.update();

        current_scene.update(&mut ctx);

        current_scene.draw(&mut ctx);

        if ctx.request_quit {
            break;
        }

        if let Some(escene) = ctx.switch_scene_to.take() {
            current_scene = match escene {
                EScene::MainMenu => Box::new(MainMenu::new(&mut ctx).await),
                EScene::Gameplay(game_options) => Box::new(Gameplay::new(&mut ctx, game_options)),
            };
        }

        next_frame().await;
    }
}
