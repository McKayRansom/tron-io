use std::collections::HashMap;

use macroquad::color::colors;
use macroquad::prelude::*;
use tron_io_world::WorldState;
use tron_io_world::client::{WorldClient, WorldEvent};

use crate::context::Context;
use crate::draw::{draw_grid, draw_rect, draw_rect_lines};
use crate::input::InputType;
use crate::scene::{GameOptions, Scene};
use crate::text::{draw_text, draw_text_screen_centered, measure_text, text_size};
use crate::{audio::SoundFx, colors::get_color};
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
    fn update(&mut self, ctx: &mut Context) {
        for (action, input_type) in ctx.input.actions.iter() {
            // Edgde case: handle somewhere else?
            if *action == input::Action::Cancel
                && matches!(self.client.game_state, WorldState::GameOver(_))
            {
                ctx.switch_scene_to = Some(crate::scene::EScene::MainMenu);
                ctx.audio.play_sfx(SoundFx::MenuCancel);
            }

            let player_id = self.input_map.get(input_type);
            if let Some(new_player_id) = self.client.handle_input(player_id.copied(), *action) {
                if player_id.is_none() {
                    self.input_map.insert(*input_type, new_player_id);
                    ctx.audio.play_sfx(SoundFx::MenuSelect);
                }
            }
        }
        self.client.update(get_time());
        while let Some(event) = self.client.events.pop() {
            match event {
                WorldEvent::PlayerJoin => ctx.audio.play_sfx(SoundFx::MenuSelect),
                WorldEvent::PlayerReady => ctx.audio.play_sfx(SoundFx::MenuSelect),
                WorldEvent::LocalUpdate(bike_update) => {
                    if bike_update.boost {
                        ctx.audio.play_sfx(SoundFx::Boost);
                    } else {
                        // turn SFX
                        ctx.audio.play_sfx(SoundFx::Turn);
                    }
                }
                WorldEvent::GameState(world_state) => match world_state {
                    WorldState::Waiting => {}
                    WorldState::Playing => {
                        // start match noise or countdown?
                        ctx.audio.play_sfx(SoundFx::RoundStart);
                    }
                    WorldState::RoundOver(winner) => {
                        if let Some(winner) = winner {
                            if let Some(_player) = self.client.local_player(winner) {
                                ctx.audio.play_sfx(SoundFx::RoundWin);
                            } else {
                                ctx.audio.play_sfx(SoundFx::RoundLose);
                            }
                        }
                    }
                    WorldState::GameOver(winner) => {
                        if let Some(_player) = self.client.local_player(winner) {
                            ctx.audio.play_sfx(SoundFx::GameWin);
                        } else {
                            ctx.audio.play_sfx(SoundFx::GameLose);
                        }
                    }
                },
                WorldEvent::ServerUpdate(grid_update_msg) => {
                    for update in &grid_update_msg.updates {
                        // only do these for remote players, local players are played immedialty
                        if self.client.local_player(update.id).is_none() {
                            if update.boost {
                                ctx.audio.play_sfx_vol(SoundFx::Boost, 0.75);
                            } else {
                                // turn SFX
                                ctx.audio.play_sfx_vol(SoundFx::Turn, 0.75);
                            }
                        }
                    }
                }
            }
        }
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
            let (text, color) = if let Some(player) = self.client.local_players.get(i) {
                (
                    format!("{} ready: {} boost: XXX", player.name, player.ready),
                    get_color(
                        self.client.grid.bikes
                            [self.client.server_player(i as u8).unwrap_or(0) as usize]
                            .get_color(),
                    ),
                )
            } else if self.client.game_state == WorldState::Waiting {
                ("[ENTER/LSHIFT/A] to join".into(), colors::WHITE)
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
                color,
            );
        }

        // draw game over
        if matches!(
            self.client.game_state,
            WorldState::GameOver(_) | WorldState::Waiting | WorldState::RoundOver(_)
        ) {
            let size: f32 = ctx.screen_size.min_element();
            draw_rectangle(
                (ctx.screen_size.x - size) / 2.,
                (ctx.screen_size.y - size) / 2.,
                size,
                size,
                Color {
                    r: 0.,
                    g: 0.,
                    b: 0.,
                    a: 0.4,
                },
            );
            let mut color = colors::WHITE;
            let (text, subtext): (String, String) = match self.client.game_state {
                WorldState::GameOver(winner) => (
                    if let Some(local_player) = self.client.local_player(winner) {
                        color = get_color(self.client.grid.get_color(winner));
                        format!("Player P{} won!", local_player)
                    } else {
                        "Game Lost!".into()
                    },
                    "Press [enter] to reboot or [delete] to exit.".into(),
                ),
                WorldState::Waiting => (
                    "Waiting for players...".into(),
                    "Press [enter] to start.".into(),
                ),
                WorldState::RoundOver(winner) => (
                    if winner.is_none() {
                        "Round tied".into()
                    } else if let Some(local_player) = self.client.local_player(winner.unwrap()) {
                        color = get_color(self.client.grid.get_color(winner.unwrap()));
                        format!("Round won by P{}", local_player)
                    } else {
                        "Round Lost...".into()
                    },
                    "Press [enter] when ready.".into(),
                ),
                _ => unreachable!(),
            };

            let mut pos: Vec2 = vec2(ctx.screen_size.x / 2., 200.);

            draw_text_screen_centered(&ctx, text.as_str(), pos.y, text::Size::Medium, color);
            pos.y += 100.;
            draw_text_screen_centered(
                &ctx,
                subtext.as_str(),
                pos.y,
                text::Size::Small,
                colors::WHITE,
            );
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
