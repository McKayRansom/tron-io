use context::Context;
use gameplay::Gameplay;
use macroquad::{
    audio::PlaySoundParams,
    color::Color,
    window::{Conf, clear_background, next_frame},
};
use scene::{EScene, main_menu::MainMenu};

// mod bike;
mod gameplay;
// mod grid;
mod assets_path;
mod audio;
mod colors;
mod context;
mod draw;
mod input;
mod online;
mod scene;
mod text;
mod ui;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const PKG_NAME: &str = env!("CARGO_PKG_NAME");

pub const BACKGROUND_COLOR: Color = Color {
    r: 0.07,
    g: 0.07,
    b: 0.07,
    a: 1.0,
};

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
    #[cfg(target_arch = "wasm32")]
    sapp_console_log::init().unwrap();
    #[cfg(not(target_arch = "wasm32"))]
    {
        // Initialize logging, and log the "info" level for this crate only, unless
        // the environment contains `RUST_LOG`.
        let env = env_logger::Env::new().default_filter_or("tron_io=info");
        env_logger::Builder::from_env(env)
            .format_module_path(false)
            .format_timestamp(None)
            .init();
    }

    log::info!("Starting {} v{}", PKG_NAME, VERSION);

    clear_background(BACKGROUND_COLOR);

    // loading assets can take a while
    let mut ctx = Context::default().await;

    ctx.audio
        .play_sfx_ex(crate::audio::SoundFx::TitleMusic, PlaySoundParams {
            looped: true,
            volume: 1.0,
        });

    let mut current_scene: Box<dyn scene::Scene> =
        Box::new(scene::main_menu::MainMenu::new(&mut ctx).await);

    loop {
        // clear_background(colors::BLACK);

        clear_background(BACKGROUND_COLOR);
        ctx.update();

        current_scene.update(&mut ctx);

        current_scene.draw(&mut ctx);

        ctx.input.virtual_gamepad.draw(&ctx);

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
