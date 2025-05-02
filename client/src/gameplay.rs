use macroquad::color::colors;
use macroquad::prelude::*;
use tron_io_world::client::WorldClient;
use tron_io_world::WorldState;

use crate::context::Context;
use crate::draw::draw_grid;
use crate::scene::{GameOptions, Scene};
use crate::text::draw_text_screen_centered;
use crate::{input, text};

pub struct Gameplay {
    client: WorldClient,
}

impl Gameplay {
    pub fn new(_context: &Context, gameoptions: GameOptions) -> Self {
        Self {
            client: gameoptions.client,
        }
    }
}

impl Scene for Gameplay {
    fn update(&mut self, context: &mut Context) {
        for action in context.input.actions.iter() {
            // Edgde case: handle somewhere else?
            if *action == input::Action::Cancel && matches!(self.client.game_state, WorldState::GameOver(_))
            {
                context.switch_scene_to = Some(crate::scene::EScene::MainMenu);
            }

            self.client.handle_input(*action);
        }
        self.client.update(get_time());
    }

    fn draw(&mut self, context: &mut Context) {
        draw_grid(&self.client.grid);
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
            let color = crate::colors::get_color(self.client.grid.bikes[i].get_color());
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
                        ("Game Won!", "Press [enter] to reboot or [delete] to exit.")
                    } else {
                        ("Game Lost.", "Press [enter] to reboot or [delete] to exit.")
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

            draw_text_screen_centered(
                &context,
                text,
                context.screen_size.y / 2.,
                text::Size::Medium,
                colors::WHITE,
            );
            draw_text_screen_centered(
                &context,
                subtext,
                context.screen_size.y / 2. + 100.,
                text::Size::Small,
                colors::WHITE,
            );
        }
    }
}
