use context::Context;
use gameplay::Gameplay;
use macroquad::{
    camera::{set_camera, set_default_camera, Camera2D},
    color::{Color, BLACK, WHITE},
    math::{vec2, Rect},
    prelude::{collections::storage, coroutines::start_coroutine, gl_use_material},
    text::draw_text,
    texture::{draw_texture_ex, DrawTextureParams},
    time::get_time,
    window::{clear_background, next_frame, screen_height, screen_width, Conf},
};
use scene::{EScene, main_menu::MainMenu};

use crate::context::{VIRTUAL_HEIGHT, VIRTUAL_WIDTH};

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
mod settings;
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

pub async fn load() -> Result<(), macroquad::Error> {
    let resources_loading = start_coroutine(async move {
        let ctx = Context::default().await;
        storage::store(ctx);
    });

    while !resources_loading.is_done() {
        clear_background(BLACK);
        let text = format!("Booting {}", ".".repeat(((get_time() * 2.) as usize) % 4));
        draw_text(
            &text,
            screen_width() / 2. - 160.,
            screen_height() / 2.,
            40.,
            WHITE,
        );
        next_frame().await;
    }

    Ok(())
}

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
        let env = env_logger::Env::new().default_filter_or("info");
        env_logger::Builder::from_env(env)
            .format_module_path(false)
            .format_timestamp(None)
            .init();
    }

    log::info!("Starting {} v{}", PKG_NAME, VERSION);

    clear_background(BACKGROUND_COLOR);

    // loading assets can take a while
    load().await.unwrap();
    let mut ctx = storage::get_mut::<Context>();

    ctx.render_target.texture.set_filter(macroquad::texture::FilterMode::Nearest);

    let mut current_scene: Box<dyn scene::Scene> =
        Box::new(scene::main_menu::MainMenu::new(&mut ctx).await);

    loop {
        let mut camera =
            Camera2D::from_display_rect(Rect::new(0.0, 0.0, VIRTUAL_WIDTH, VIRTUAL_HEIGHT));
        camera.render_target = Some(ctx.render_target.clone());

        set_camera(&camera);

        clear_background(BACKGROUND_COLOR);

        ctx.update();

        current_scene.update(&mut ctx);

        current_scene.draw(&mut ctx);

        ctx.input.virtual_gamepad.draw(&ctx);

        // regular drawing
        set_default_camera();
        clear_background(BACKGROUND_COLOR); // Will be the letterbox color

        // set_ma

        gl_use_material(&ctx.crt_material);

        // draw the render target properly scaled and letterboxed
        let scale: f32 = f32::min(
            screen_width() / VIRTUAL_WIDTH,
            screen_height() / VIRTUAL_HEIGHT,
        );
        draw_texture_ex(
            &ctx.render_target.texture,
            (screen_width() - (VIRTUAL_WIDTH * scale)) * 0.5,
            (screen_height() - (VIRTUAL_HEIGHT * scale)) * 0.5,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(VIRTUAL_WIDTH * scale, VIRTUAL_HEIGHT * scale)),
                flip_y: true, // Must flip y otherwise 'render_target' will be upside down
                ..Default::default()
            },
        );

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
