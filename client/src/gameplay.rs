use std::collections::HashMap;

use macroquad::color::colors;
use macroquad::prelude::*;
use tron_io_world::WorldState;
use tron_io_world::client::WorldClient;

use crate::colors::get_color;
use crate::context::Context;
use crate::draw::{draw_grid, draw_rect, draw_rect_lines};
use crate::input::InputType;
use crate::scene::{GameOptions, Scene};
use crate::text::{draw_text, draw_text_screen_centered, measure_text, text_size};
use crate::{input, text};

pub struct Gameplay {
    input_map: HashMap<InputType, u8>,
    client: WorldClient,
}

impl Gameplay {
    pub fn new(_context: &Context, gameoptions: GameOptions) -> Self {
        Self {
            input_map: HashMap::new(),
            client: gameoptions.client,
        }
    }
}

impl Scene for Gameplay {
    fn update(&mut self, context: &mut Context) {
        for (action, input_type) in context.input.actions.iter() {
            // Edgde case: handle somewhere else?
            if *action == input::Action::Cancel
                && matches!(self.client.game_state, WorldState::GameOver(_))
            {
                context.switch_scene_to = Some(crate::scene::EScene::MainMenu);
            }

            let player_id = self.input_map.get(input_type);
            if let Some(new_player_id) = self.client.handle_input(player_id.copied(), *action) {
                if player_id.is_none() {
                    self.input_map.insert(*input_type, new_player_id);
                }
            }
        }
        self.client.update(get_time());
    }

    fn draw(&mut self, ctx: &mut Context) {
        draw_grid(&self.client.grid);

        // draw players in corners
        for i in 0..4 {
            const PLAYER_EDGE_SPACING: f32 = 24.;
            let pos = match i {
                0 => vec2(PLAYER_EDGE_SPACING, PLAYER_EDGE_SPACING),
                1 => vec2(ctx.screen_size.x - PLAYER_EDGE_SPACING, PLAYER_EDGE_SPACING),
                2 => vec2(PLAYER_EDGE_SPACING, ctx.screen_size.y - PLAYER_EDGE_SPACING),
                3 => vec2(
                    ctx.screen_size.x - PLAYER_EDGE_SPACING,
                    ctx.screen_size.y - PLAYER_EDGE_SPACING,
                ),
                _ => unreachable!(),
            };
            let text: String = if let Some(player) = self.client.local_players.get(i) {
                format!("{} ready: {} boost: XXX", player.name, player.ready)
            } else if self.client.game_state == WorldState::Waiting {
                "[ENTER/LSHIFT/A] to join".into()
            } else {
                continue;
            };
            let measure = measure_text(ctx, &text, text::Size::Small);
            draw_text(
                ctx,
                &text,
                if i % 2 == 0 {
                    pos.x
                } else {
                    pos.x - measure.width
                },
                pos.y,
                text::Size::Small,
                colors::WHITE,
            );
        }

        // draw game over
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
                WorldState::GameOver(_winner) => {
                    ("Game Over!", "Press [enter] to reboot or [delete] to exit.")
                }
                WorldState::Waiting => ("Waiting for players...", "Press [enter] to start."),
                WorldState::RoundOver(_winner) => ("Round Over", "Press [enter] when ready."),
                _ => unreachable!(),
            };

            let mut pos: Vec2 = vec2(ctx.screen_size.x / 2., 200.);

            draw_text_screen_centered(&ctx, text, pos.y, text::Size::Medium, colors::WHITE);
            pos.y += 100.;
            draw_text_screen_centered(&ctx, subtext, pos.y, text::Size::Small, colors::WHITE);
            pos.y += 100.;

            for (i, player) in self.client.server_players.iter().enumerate() {
                let color = get_color(self.client.grid.bikes[i].get_color());
                let measure = measure_text(&ctx, &player.name, text::Size::Large);
                let box_size = text_size(text::Size::Medium);
                const SCORE_CENTER_PAD: f32 = 10.;
                draw_text(
                    &ctx,
                    &player.name,
                    pos.x - measure.width - SCORE_CENTER_PAD,
                    pos.y - measure.height / 2.,
                    text::Size::Large,
                    color,
                );
                for score in 0..self.client.score_win {
                    let rect: Rect = Rect::new(
                        pos.x + SCORE_CENTER_PAD + score as f32 * (box_size + 10) as f32,
                        pos.y - (box_size as f32) / 2. - measure.height,
                        box_size as f32,
                        box_size as f32,
                    );
                    // score is 0-based but we want to draw 1-based
                    if score < player.score {
                        draw_rect(rect, color);
                    } else {
                        draw_rect_lines(rect, 8., color);
                    }
                }
                pos.y += 50.;
            }
        }
    }
}
