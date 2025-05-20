use nanoserde::{DeRon, SerRon};
use quad_lib::storage::Storage;

pub const SOUND_MIN: u8 = 0;
pub const SOUND_MAX: u8 = 10;

const STORAGE: Storage = Storage::new("game-settings", ".ron");

#[derive(DeRon, SerRon, Debug, Clone)]
pub struct GameSettings {
    pub sound: u8, // 0..10
    pub music: u8, // 0..10
    pub fullscreen: bool,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            sound: SOUND_MAX,
            music: SOUND_MAX,
            fullscreen: false,
        }
    }
}

impl GameSettings {
    pub fn load() -> Self {
        if let Ok(str) = STORAGE.load() {
            DeRon::deserialize_ron(&str).unwrap_or_else(|err| {
                log::error!("Error when deser: {:?}", err);
                Self::default()
            })
        } else {
            log::info!("No settings found, loading defaults");
            Self::default()
        }
    }

    pub fn save(&mut self) {
        if let Err(err) = STORAGE.save(SerRon::serialize_ron(self).as_str()) {
            log::error!("Error saving: {:?}", err);
        }
    }
}
