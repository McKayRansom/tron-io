use gamepads::Gamepads;
use macroquad::input::{get_keys_pressed, KeyCode};

pub mod virtual_gamepad;

pub use tron_io::world::Action;

pub struct InputContext {
    pub actions: Vec<Action>,
    gamepads: Gamepads,
    pub virtual_gamepad: virtual_gamepad::VirtualGamepad,
}

impl InputContext {
    pub fn new() -> Self {
        Self {
            actions: Vec::new(),
            gamepads: Gamepads::new(),
            virtual_gamepad: virtual_gamepad::VirtualGamepad::new(),
        }
    }

    fn keycode_to_action(key: KeyCode) -> Option<Action> {
        match key {
            KeyCode::W | KeyCode::Up => Some(Action::Up),
            KeyCode::S | KeyCode::Down => Some(Action::Down),
            KeyCode::A | KeyCode::Left => Some(Action::Left),
            KeyCode::D | KeyCode::Right => Some(Action::Right),
            KeyCode::X => Some(Action::Rewind),
            KeyCode::L | KeyCode::C => Some(Action::Reset),
            KeyCode::J | KeyCode::Z | KeyCode::Enter => Some(Action::Confirm),
            KeyCode::K | KeyCode::Delete | KeyCode::Backspace => Some(Action::Cancel),
            _ => None,
        }
    }

    fn gamepad_to_action(button: gamepads::Button) -> Option<Action> {
        match button {
            gamepads::Button::DPadUp => Some(Action::Up),
            gamepads::Button::DPadDown => Some(Action::Down),
            gamepads::Button::DPadLeft => Some(Action::Left),
            gamepads::Button::DPadRight => Some(Action::Right),
            gamepads::Button::ActionUp => Some(Action::Reset),
            gamepads::Button::ActionDown => Some(Action::Confirm),
            gamepads::Button::ActionLeft => Some(Action::Rewind),
            gamepads::Button::ActionRight => Some(Action::Cancel),
            gamepads::Button::RightCenterCluster => Some(Action::Pause),
            _ => None,
        }
    }

    pub fn update(&mut self) {
        self.actions.clear();

        if let Some(action) = self.virtual_gamepad.update() {
            self.actions.push(action);
        }
        for key in get_keys_pressed() {
            if let Some(action) = Self::keycode_to_action(key) {
                self.actions.push(action);
            }
        }
        self.gamepads.poll();

        for gamepad in self.gamepads.all() {
            for button in gamepad.all_just_pressed() {
                if let Some(action) = Self::gamepad_to_action(button) {
                    self.actions.push(action);
                }
            }
        }
    }
}
