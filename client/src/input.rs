use gamepads::{GamepadId, Gamepads};
use macroquad::input::{get_keys_pressed, KeyCode};

pub mod virtual_gamepad;

pub use tron_io_world::Action;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum InputType {
    Keyboard(u8),
    Gamepad(GamepadId),
    VirtualTouch,
}


pub struct InputContext {
    pub actions: Vec<(Action, InputType)>,
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

    fn keycode_to_action(key: KeyCode) -> Option<(Action, u8)> {
        match key {
            KeyCode::W => Some((Action::Up, 0)),
            KeyCode::S => Some((Action::Down, 0)),
            KeyCode::A => Some((Action::Left, 0)),
            KeyCode::D => Some((Action::Right, 0)),
            KeyCode::Z => Some((Action::Rewind, 0)),
            KeyCode::L | KeyCode::C => Some((Action::Reset, 0)),
            KeyCode::LeftShift | KeyCode::J => Some((Action::Confirm, 0)),
            KeyCode::Enter | KeyCode::X  => Some((Action::Confirm, 1)),
            KeyCode::Up => Some((Action::Up, 1)),
            KeyCode::Down => Some((Action::Down, 1)),
            KeyCode::Left => Some((Action::Left, 1)),
            KeyCode::Right => Some((Action::Right, 1)),
            KeyCode::K | KeyCode::Delete | KeyCode::Backspace => Some((Action::Cancel, 0)),
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
            self.actions.push((action, InputType::VirtualTouch));
        }
        for key in get_keys_pressed() {
            if let Some((action, id)) = Self::keycode_to_action(key) {
                self.actions.push((action, InputType::Keyboard(id)));
            }
        }
        self.gamepads.poll();

        for gamepad in self.gamepads.all() {
            for button in gamepad.all_just_pressed() {
                if let Some(action) = Self::gamepad_to_action(button) {
                    self.actions.push((action, InputType::Gamepad(gamepad.id())));
                }
            }
        }
    }
}
