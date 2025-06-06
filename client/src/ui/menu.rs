use macroquad::{color::colors, math::Vec2};
use tron_io_world::Action;

use crate::{audio::SoundFx, context::Context, text};

pub const MENU_SPACING: f32 = 40.;

pub struct Menu {
    pub pos: Vec2,
    pub selected: usize,
    pub current: usize,
}

const SELECT_MAX: usize = 128;

pub enum MenuAction {
    Enter,
    Right,
    Left,
}

impl Menu {
    pub fn new(pos: Vec2, selected: usize) -> Self {
        Self {
            pos,
            selected,
            current: 0,
        }
    }

    pub fn option(&mut self, name: &str, ctx: &mut Context) -> bool {
        matches!(self.option_ex(name, ctx), Some(MenuAction::Enter))
    }

    pub fn option_ex(&mut self, name: &str, ctx: &mut Context) -> Option<MenuAction> {
        let mut action: Option<MenuAction> = None;
        let mut selected = false;
        if self.selected == self.current {
            selected = true;
            for (input, _input_type) in ctx.input.actions.iter() {
                match input {
                    Action::Up => {
                        ctx.audio.play_sfx(SoundFx::MenuMove);
                        if self.selected == 0 {
                            self.selected = SELECT_MAX;
                        } else {
                            self.selected -= 1;
                        }
                    }
                    Action::Down => {
                        ctx.audio.play_sfx(SoundFx::MenuMove);
                        self.selected += 1;
                    }
                    Action::Confirm => {
                        ctx.audio.play_sfx(SoundFx::MenuSelect);
                        action = Some(MenuAction::Enter);
                    }
                    Action::Left => {
                        ctx.audio.play_sfx(SoundFx::MenuSelect);
                        action = Some(MenuAction::Left)
                    }
                    Action::Right => {
                        ctx.audio.play_sfx(SoundFx::MenuSelect);
                        action = Some(MenuAction::Right)
                    }
                    _ => {}
                }
            }
            ctx.input.actions.clear();
        }
        let color = if selected {
            colors::GREEN
        } else {
            colors::WHITE
        };

        // let label = if let Some(setting) = &self.setting {
        //     &format!("<{}: {}>", self.text, setting.options[setting.selected])
        // } else {
        //     &self.text
        // };

        text::draw_text_screen_centered(ctx, name, self.pos.y, text::Size::Medium, color);

        self.pos.y += MENU_SPACING;
        self.current += 1;

        action
    }

    // this seems wrong...
    pub fn finish(&mut self) -> usize {
        if self.current == self.selected {
            self.selected = 0; // is this desired behaviour?
        } else if self.selected == SELECT_MAX {
            self.selected = self.current - 1;
        }
        self.selected
    }

    // pub fn draw(&mut self, ctx: &mut Context, mut pos: Vec2) -> Option<&V> {
    //     for (i, item) in self.items.iter().enumerate() {
    //         item.draw(ctx, pos, i == self.selected);
    //         pos.y += MENU_SPACING;
    //     }

    //     None
    // }
}
