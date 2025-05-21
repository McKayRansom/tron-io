use std::path::Path;

use macroquad::audio::{PlaySoundParams, Sound};

use crate::settings::SOUND_MAX;

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
    Explosion,
    TitleMusic,
}
pub struct AudioAtlas {
    pub sound_volume: f32,
    pub sfx: Vec<(SoundFx, Sound)>,
    last_sound: Option<SoundFx>,
}

async fn load_sfx(base_path: &Path, path: &str) -> Sound {
    macroquad::audio::load_sound(base_path.join(path).to_str().unwrap())
        .await
        .unwrap()
}

impl AudioAtlas {
    pub async fn new(base: &Path, settings: &crate::settings::GameSettings) -> Self {
        let mut this = Self {
            sound_volume: 1.0,
            sfx: vec![
                // Dang, this sucks, safari on IOS might not support .ogg... Didn't actually test it tho
                #[cfg(target_arch = "wasm32")]
                (
                    SoundFx::TitleMusic,
                    load_sfx(base, "sfx/jamuary-2023-01.mp3").await,
                ),
                #[cfg(not(target_arch = "wasm32"))]
                (
                    SoundFx::TitleMusic,
                    load_sfx(base, "sfx/jamuary-2023-01.ogg").await,
                ),
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
                (SoundFx::Explosion, load_sfx(base, "sfx/explosion.wav").await),
                // (SoundFx::MenuCancel, load_sfx(base, "sfx/menuCancel.wav").await),
                // (SoundFx::MenuCancel, load_sfx(base, "sfx/menuCancel.wav").await),
            ],
            last_sound: None,
        };

        this.play_sfx_ex(crate::audio::SoundFx::TitleMusic, PlaySoundParams {
            looped: true,
            volume: 1.0,
        });
        this.settings(settings);
        this
    }

    pub fn update(&mut self) {
        self.last_sound = None;
    }

    pub fn play_sfx(&mut self, effect: SoundFx) {
        self.play_sfx_ex(effect, PlaySoundParams::default());
    }

    pub fn play_sfx_vol(&mut self, effect: SoundFx, volume: f32) {
        self.play_sfx_ex(effect, PlaySoundParams {
            looped: false,
            volume,
        });
    }

    pub fn play_sfx_ex(&mut self, effect: SoundFx, mut params: PlaySoundParams) {
        if self.sound_volume == 0.0 || Some(effect) == self.last_sound {
            return;
        }
        self.last_sound = Some(effect);
        params.volume *= self.sound_volume;
        for sfx in &self.sfx {
            if sfx.0 == effect {
                macroquad::audio::play_sound(&sfx.1, params);
                return;
            }
        }
        panic!("No Sound file for SFX: {:?}", effect);
    }

    pub(crate) fn settings(&mut self, settings: &crate::settings::GameSettings) {
        self.sound_volume = settings.sound as f32 / SOUND_MAX as f32;
        let music_volume = settings.music as f32 / SOUND_MAX as f32;
        macroquad::audio::set_sound_volume(&self.sfx[0].1, music_volume);
    }
}
