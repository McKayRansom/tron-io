use std::path::Path;

use macroquad::audio::{PlaySoundParams, Sound};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SoundFx {
    MenuCancel,
    MenuSelect,
    MenuMove,
    RoundStart,
    RoundWin,
    RoundLose,
    GameWin,
    GameLose,
    Turn,
    Boost,
}
pub struct AudioAtlas {
    pub sfx: Vec<(SoundFx, Sound)>,
}

async fn load_sfx(base_path: &Path, path: &str) -> Sound {
    macroquad::audio::load_sound(base_path.join(path).to_str().unwrap())
        .await
        .unwrap()
}

impl AudioAtlas {
    pub async fn new(base: &Path) -> Self {
        Self {
            sfx: vec![
                (
                    SoundFx::MenuCancel,
                    load_sfx(base, "sfx/menuCancel.wav").await,
                ),
                (
                    SoundFx::MenuSelect,
                    load_sfx(base, "sfx/blipEnter.wav").await,
                ),
                (
                    SoundFx::MenuMove,
                    load_sfx(base, "sfx/blipSelect.wav").await,
                ),
                (SoundFx::RoundWin, load_sfx(base, "sfx/roundWin.wav").await),
                (SoundFx::Turn, load_sfx(base, "sfx/turn.wav").await),
                (SoundFx::Boost, load_sfx(base, "sfx/boost.wav").await),
                (
                    SoundFx::RoundStart,
                    load_sfx(base, "sfx/roundStart.wav").await,
                ),
                (
                    SoundFx::RoundLose,
                    load_sfx(base, "sfx/roundLose.wav").await,
                ),
                (SoundFx::GameWin, load_sfx(base, "sfx/gameWin.wav").await),
                (SoundFx::GameLose, load_sfx(base, "sfx/gameLose.wav").await),
                // (SoundFx::MenuCancel, load_sfx(base, "sfx/menuCancel.wav").await),
                // (SoundFx::MenuCancel, load_sfx(base, "sfx/menuCancel.wav").await),
            ],
        }
    }

    pub fn play_sfx(&self, effect: SoundFx) {
        // if ctx.settings.is_muted() {
        //     return;
        // }
        for sfx in &self.sfx {
            if sfx.0 == effect {
                macroquad::audio::play_sound_once(&sfx.1);
                return;
            }
        }
        panic!("No Sound file for SFX: {:?}", effect);
    }

    pub fn play_sfx_vol(&self, effect: SoundFx, vol: f32) {
        // if ctx.settings.is_muted() {
        //     return;
        // }
        for sfx in &self.sfx {
            if sfx.0 == effect {
                macroquad::audio::play_sound(&sfx.1, PlaySoundParams {
                    looped: false,
                    volume: vol,
                });
                return;
            }
        }
        panic!("No Sound file for SFX: {:?}", effect);
    }
}
