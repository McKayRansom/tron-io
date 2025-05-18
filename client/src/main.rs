use context::Context;
use gameplay::Gameplay;
use macroquad::{
    audio::PlaySoundParams, camera::{set_camera, set_default_camera, Camera2D}, color::{Color, BLACK, WHITE}, math::vec2, prelude::{
        collections::storage, coroutines::start_coroutine, gl_use_default_material, gl_use_material, load_material, ShaderSource
    }, text::draw_text, texture::{draw_texture_ex, render_target, DrawTextureParams}, time::get_time, window::{clear_background, next_frame, screen_height, screen_width, Conf}
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

pub async fn load() -> Result<(), macroquad::Error> {
    let resources_loading = start_coroutine(async move {
        let ctx = Context::default().await;
        storage::store(ctx);
    });

    while !resources_loading.is_done() {
        clear_background(BLACK);
        let text = format!("Loading {}", ".".repeat(((get_time() * 2.) as usize) % 4));
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
        let env = env_logger::Env::new().default_filter_or("tron_io=info");
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

    ctx.audio
        .play_sfx_ex(crate::audio::SoundFx::TitleMusic, PlaySoundParams {
            looped: true,
            volume: 1.0,
        });

    let mut current_scene: Box<dyn scene::Scene> =
        Box::new(scene::main_menu::MainMenu::new(&mut ctx).await);

    let material = load_material(
        ShaderSource::Glsl {
            // vertex: GLOW_VERTEX_SHADER,
            vertex: CRT_VERTEX_SHADER,
            fragment: CRT_FRAGMENT_SHADER,
        },
        Default::default(),
    )
    .unwrap();

    // storage::store(material);

    let render_target = render_target(screen_width() as u32, screen_height() as u32);
    render_target
        .texture
        .set_filter(macroquad::texture::FilterMode::Nearest);

    loop {
        // clear_background(BLACK);

        // clear_background(BACKGROUND_COLOR);
        set_camera(&Camera2D {
            zoom: vec2(0.0015, 0.0025),
            target: vec2(screen_width() / 2., screen_height() / 2.),
            render_target: Some(render_target.clone()),
            ..Default::default()
        });
        clear_background(BLACK);

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
        set_default_camera();
        clear_background(WHITE);
        gl_use_material(&material);
        draw_texture_ex(&render_target.texture, 0., 0., WHITE, DrawTextureParams {
            // dest_size: Some(vec2(screen_width(), screen_height())),
            ..Default::default()
        });
        gl_use_default_material();
        next_frame().await;
    }
}

/*
 * Macroquad seems to know about 'Model' and 'Projection' uniforms
 * 
 * and it creates position, texcoord, color0, normal at some point as well
 * 
 */
pub const VERTEX: &str = r#"#version 100
    attribute vec3 position;
    attribute vec2 texcoord;
    attribute vec4 color0;
    attribute vec4 normal;

    varying lowp vec2 uv;
    varying lowp vec4 color;

    uniform mat4 Model;
    uniform mat4 Projection;

    void main() {
        gl_Position = Projection * Model * vec4(position, 1);
        color = color0 / 255.0;
        uv = texcoord;
    }"#;

// Macroquad seems to create 'Texture' and '_ScreenTexture' images (but I guess they show as uniforms?)
pub const FRAGMENT: &str = r#"#version 100
    varying lowp vec4 color;
    varying lowp vec2 uv;

    uniform sampler2D Texture;

    void main() {
        gl_FragColor = color * texture2D(Texture, uv) ;
    }"#;

/**
------------ one pass glow shader ------------

    author: Richman Stewart

    applies a gaussian glow horizontally and vertically
    behind the original texture

------------------ use ------------------------

    glow_size - defines the spread x and y
    glow_colour - the colour of the glow
    glow_intensity - glow intensity

**/

const GLOW_FRAGMENT_SHADER: &'static str = r#"#version 140

/**
------------ one pass glow shader ------------

    author: Richman Stewart

    applies a gaussian glow horizontally and vertically
    behind the original texture

------------------ use ------------------------

    glow_size - defines the spread x and y
    glow_colour - the colour of the glow
    glow_intensity - glow intensity

**/

in vec4 v_colour;
in vec2 tex_coord;
out vec4 pixel;

uniform sampler2D t0;
uniform float glow_size = .5;
uniform vec3 glow_colour = vec3(0, 0, 0);
uniform float glow_intensity = 1;
uniform float glow_threshold = .5;

void main() {
    pixel = texture(t0, tex_coord);
    if (pixel.a <= glow_threshold) {
        ivec2 size = textureSize(t0, 0);
	
        float uv_x = tex_coord.x * size.x;
        float uv_y = tex_coord.y * size.y;

        float sum = 0.0;
        for (int n = 0; n < 9; ++n) {
            uv_y = (tex_coord.y * size.y) + (glow_size * float(n - 4.5));
            float h_sum = 0.0;
            h_sum += texelFetch(t0, ivec2(uv_x - (4.0 * glow_size), uv_y), 0).a;
            h_sum += texelFetch(t0, ivec2(uv_x - (3.0 * glow_size), uv_y), 0).a;
            h_sum += texelFetch(t0, ivec2(uv_x - (2.0 * glow_size), uv_y), 0).a;
            h_sum += texelFetch(t0, ivec2(uv_x - glow_size, uv_y), 0).a;
            h_sum += texelFetch(t0, ivec2(uv_x, uv_y), 0).a;
            h_sum += texelFetch(t0, ivec2(uv_x + glow_size, uv_y), 0).a;
            h_sum += texelFetch(t0, ivec2(uv_x + (2.0 * glow_size), uv_y), 0).a;
            h_sum += texelFetch(t0, ivec2(uv_x + (3.0 * glow_size), uv_y), 0).a;
            h_sum += texelFetch(t0, ivec2(uv_x + (4.0 * glow_size), uv_y), 0).a;
            sum += h_sum / 9.0;
        }

        pixel = vec4(glow_colour, (sum / 9.0) * glow_intensity);
    }
}
"#;

/**
-------------- glow vertex shader -------------

    author: Richman Stewart

    simple vertex shader that sets the position
    to the specified matrix and position while
    passing the vertex colour and tex coords
    to the fragment shader

**/
const GLOW_VERTEX_SHADER: &'static str = r#"#version 140

/**
-------------- glow vertex shader -------------

    author: Richman Stewart

    simple vertex shader that sets the position
    to the specified matrix and position while
    passing the vertex colour and tex coords
    to the fragment shader

**/

in vec2 a_position;
in vec2 a_tex_coord;
in vec4 a_colour;

uniform mat4 matrix;

out vec4 v_colour;
out vec2 tex_coord;

void main() {
   v_colour = a_colour;
   tex_coord = a_tex_coord;
   gl_Position = matrix * vec4(a_position, 0, 1);
}

"#;


const CRT_FRAGMENT_SHADER: &'static str = r#"#version 100
precision lowp float;

varying vec4 color;
varying vec2 uv;

uniform sampler2D Texture;

// https://www.shadertoy.com/view/XtlSD7

vec2 CRTCurveUV(vec2 uv)
{
    uv = uv * 2.0 - 1.0;
    vec2 offset = abs( uv.yx ) / vec2( 6.0, 4.0 );
    uv = uv + uv * offset * offset;
    uv = uv * 0.5 + 0.5;
    return uv;
}

void DrawVignette( inout vec3 color, vec2 uv )
{
    float vignette = uv.x * uv.y * ( 1.0 - uv.x ) * ( 1.0 - uv.y );
    vignette = clamp( pow( 16.0 * vignette, 0.3 ), 0.0, 1.0 );
    color *= vignette;
}


void DrawScanline( inout vec3 color, vec2 uv )
{
    float iTime = 0.1;
    float scanline 	= clamp( 0.95 + 0.05 * cos( 3.14 * ( uv.y + 0.008 * iTime ) * 240.0 * 1.0 ), 0.0, 1.0 );
    float grille 	= 0.85 + 0.15 * clamp( 1.5 * cos( 3.14 * uv.x * 640.0 * 1.0 ), 0.0, 1.0 );
    color *= scanline * grille * 1.2;
}

void main() {
    vec2 crtUV = CRTCurveUV(uv);
    vec3 res = texture2D(Texture, uv).rgb * color.rgb;
    if (crtUV.x < 0.0 || crtUV.x > 1.0 || crtUV.y < 0.0 || crtUV.y > 1.0)
    {
        res = vec3(0.0, 0.0, 0.0);
    }
    DrawVignette(res, crtUV);
    DrawScanline(res, uv);
    gl_FragColor = vec4(res, 1.0);

}
"#;


// NOTE: CRT Shader looks really good on the bikes...
const CRT_FRAGMENT_SHADER_MODIFIED: &'static str = r#"#version 100
precision lowp float;

varying vec4 color;
varying vec2 uv;

uniform sampler2D Texture;

// https://www.shadertoy.com/view/XtlSD7

vec2 CRTCurveUV(vec2 uv)
{
    uv = uv * 2.0 - 1.0;
    vec2 offset = abs( uv.yx ) / vec2( 6.0, 4.0 );
    uv = uv + uv * offset * offset;
    uv = uv * 0.5 + 0.5;
    return uv;
}

void DrawVignette( inout vec3 color, vec2 uv )
{
    float vignette = uv.x * uv.y * ( 1.0 - uv.x ) * ( 1.0 - uv.y );
    vignette = clamp( pow( 16.0 * vignette, 0.3 ), 0.0, 1.0 );
    color *= vignette;
}


void DrawScanline( inout vec3 color, vec2 uv )
{
    float iTime = 0.1;
    float scanline 	= clamp( 0.95 + 0.05 * cos( 3.14 * ( uv.y + 0.008 * iTime ) * 240.0 * 1.0 ), 0.0, 1.0 );
    float grille 	= 0.85 + 0.15 * clamp( 1.5 * cos( 3.14 * uv.x * 640.0 * 1.0 ), 0.0, 1.0 );
    color *= scanline * grille * 1.2;
}

// TODO: I guessed at const is that a real thing
const float glow_threshold = .5;
const float glow_distance = 0.0010;

void main() {
    // vec2 crtUV = CRTCurveUV(uv);
    // vec3 res = texture2D(Texture, uv).rgb * color.rgb;
    // if (crtUV.x < 0.0 || crtUV.x > 1.0 || crtUV.y < 0.0 || crtUV.y > 1.0)
    // {
    //     res = vec3(0.0, 0.0, 0.0);
    // }
    // DrawVignette(res, crtUV);
    // DrawScanline(res, uv);
    gl_FragColor = color * texture2D(Texture, uv);
    if (gl_FragColor.r <= glow_threshold && gl_FragColor.g <= glow_threshold && gl_FragColor.b <= glow_threshold) {
        vec4 sum = vec4(0.0, 0.0, 0.0, 0.0);
        for (int n = 0; n < 9; ++n) {
            // uv_y = (tex_coord.y * size.y) + (glow_size * float(n - 4.5));
            // float h_sum = 0.0;
            vec4 h_sum = vec4(0.0, 0.0, 0.0, 0.0);
            h_sum += color * texture2D(Texture, uv + vec2(glow_distance, 0.0) * vec2(n, n));
            h_sum += color * texture2D(Texture, uv + vec2(-glow_distance, 0.0) * vec2(n, n));
            h_sum += color * texture2D(Texture, uv + vec2(0.0, glow_distance) * vec2(n, n));
            h_sum += color * texture2D(Texture, uv + vec2(0.0, -glow_distance) * vec2(n, n));
            // sum += vec4(1.0, 0.0, 0.0, 0.0);

            // h_sum += texelFetch(t0, ivec2(uv_x - (4.0 * glow_size), uv_y), 0).a;
            // h_sum += texelFetch(t0, ivec2(uv_x - (3.0 * glow_size), uv_y), 0).a;
            // h_sum += texelFetch(t0, ivec2(uv_x - (2.0 * glow_size), uv_y), 0).a;
            // h_sum += texelFetch(t0, ivec2(uv_x - glow_size, uv_y), 0).a;
            // h_sum += texelFetch(t0, ivec2(uv_x, uv_y), 0).a;
            // h_sum += texelFetch(t0, ivec2(uv_x + glow_size, uv_y), 0).a;
            // h_sum += texelFetch(t0, ivec2(uv_x + (2.0 * glow_size), uv_y), 0).a;
            // h_sum += texelFetch(t0, ivec2(uv_x + (3.0 * glow_size), uv_y), 0).a;
            // h_sum += texelFetch(t0, ivec2(uv_x + (4.0 * glow_size), uv_y), 0).a;
            sum += h_sum / 4.0;
        }
        gl_FragColor = sum / 9.0;
    }

}
"#;

const CRT_VERTEX_SHADER: &'static str = "#version 100
attribute vec3 position;
attribute vec2 texcoord;
attribute vec4 color0;

varying lowp vec2 uv;
varying lowp vec4 color;

uniform mat4 Model;
uniform mat4 Projection;

void main() {
    gl_Position = Projection * Model * vec4(position, 1);
    color = color0 / 255.0;
    uv = texcoord;
}
";
