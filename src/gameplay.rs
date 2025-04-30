use macroquad::color::colors;
use macroquad::prelude::*;
use tron_io::grid::msg::WorldState;
use tron_io::world::client::WorldClient;
use tron_io::world::{self, Action};

use crate::context::Context;
use crate::scene::{GameOptions, Scene};
use crate::text::draw_text_centered;
use crate::{input, text};
use tron_io::grid::PLAYER_COLOR_LOOKUP;

pub struct Gameplay {
    // speed: f64,
    // last_update: f64,
    client: WorldClient,
}

impl Gameplay {
    pub fn new(context: &Context, gameoptions: GameOptions) -> Self {
        Self {
            // speed: 0.05,
            // last_update: get_time(),
            client: gameoptions.client,
        }
    }

    fn update_player_input(&self) -> Option<world::Action> {
        // input::action_pressed(action, gamepads)
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            Some(Action::Right)
        } else if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            Some(Action::Left)
        } else if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
            Some(Action::Up)
        } else if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
            Some(Action::Down)
        } else {
            None
        }
    }
}

impl Scene for Gameplay {
    fn update(&mut self, context: &mut Context) {
        if input::action_pressed(input::Action::Confirm, &context.gamepads) {
            self.client.handle_input(&input::Action::Confirm);
        }

        self.client.update();

        if let Some(action) = self.update_player_input() {
            self.client.handle_input(&action);
        }
    }

    fn draw(&mut self, context: &mut Context) {
        clear_background(BLACK);

        self.client.grid.draw();
        text::draw_text(context, "WN:", 10., 30., text::Size::Medium, colors::WHITE);

        const BOX_POS_ADJUSTMENT: f32 = text::text_size(text::Size::Medium) as f32 / 2.;

        for j in 0..self.client.score_win {
            draw_rectangle(
                60. + j as f32 * 20.,
                30. - BOX_POS_ADJUSTMENT,
                15.,
                15.,
                colors::WHITE,
            );
        }

        for i in 0..2 {
            let pos = vec2(10., 60. + 30. * i as f32);
            let color = PLAYER_COLOR_LOOKUP[self.client.grid.bikes[i].color as usize].0;
            text::draw_text(
                context,
                format!("P{}:", i + 1).as_str(),
                pos.x,
                pos.y,
                text::Size::Medium,
                color,
            );
            for j in 0..self.client.scores[i] {
                draw_rectangle(
                    pos.x + 50. + j as f32 * 20.,
                    pos.y - BOX_POS_ADJUSTMENT,
                    15.,
                    15.,
                    color,
                );
            }
            // text::draw_text(
            //     context,
            //     format!("{}", self.scores[i]).as_str(),
            //     pos.x + 50.,
            //     pos.y,
            //     text::Size::Medium,
            //     colors::WHITE,
            // );
        }

        if matches!(
            self.client.game_state,
            WorldState::GameOver(_) | WorldState::Waiting | WorldState::RoundOver(_)
        ) {
            draw_rectangle(0., 0., screen_width(), screen_height(), Color {
                r: 0.,
                g: 0.,
                b: 0.,
                a: 0.5,
            });
            let (text, subtext) = match self.client.game_state {
                WorldState::GameOver(winner) => {
                    if winner == self.client.player_id.unwrap() {
                        ("Game Won!", "Press [enter] to play again.")
                    } else {
                        ("Game Lost.", "Press [enter] to play again.")
                    }
                }
                WorldState::Waiting => ("Waiting for players...", "Press [enter] to start."),
                WorldState::RoundOver(winner) => (
                    if winner == self.client.player_id.unwrap() {
                        "Round Won!"
                    } else {
                        "Round Lost."
                    },
                    "Press [enter] when ready.",
                ),
                _ => unreachable!(),
            };

            draw_text_centered(
                &context,
                text,
                context.screen_size.y / 2.,
                text::Size::Medium,
                colors::WHITE,
            );
            draw_text_centered(
                &context,
                subtext,
                context.screen_size.y / 2. + 100.,
                text::Size::Small,
                colors::WHITE,
            );
        }
    }
}
