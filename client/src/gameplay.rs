use std::collections::HashMap;

use macroquad::audio::PlaySoundParams;
use macroquad::color::colors;
use macroquad::prelude::*;
use tron_io_world::client::{WorldClient, WorldEvent};
use tron_io_world::local::WorldClientLocal;
use tron_io_world::{GridOptions, WorldState};

use crate::colors::get_team_color;
use crate::context::Context;
use crate::draw::{draw_grid, draw_rect, draw_rect_lines};
use crate::input::InputType;
use crate::scene::Scene;
use crate::text::{draw_text, draw_text_screen_centered, measure_text, text_size};
use crate::{audio::SoundFx, colors::get_color};
use crate::{input, text};

pub struct Gameplay {
    input_map: HashMap<InputType, u8>,
    client: WorldClient,
}

impl Gameplay {
    pub fn new(_context: &Context, gameoptions: GridOptions) -> Self {
        Self {
            input_map: HashMap::new(),
            // client: gameoptions.client,
            client: WorldClient::new(Box::new(WorldClientLocal::new(gameoptions))),
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
                        // ctx.audio.play_sfx(SoundFx::Boost);
                        ctx.audio.play_sfx(SoundFx::Turn);
                    } else {
                        // THIS IS SUPER ANNOYING
                        // turn SFX
                        // ctx.audio.play_sfx(SoundFx::Turn);
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
                            // TODO: THIS IS WRONG!
                            if let Some(_player) = self.client.local_player(winner) {
                                ctx.audio.play_sfx(SoundFx::RoundWin);
                            } else {
                                ctx.audio.play_sfx(SoundFx::RoundLose);
                            }
                        }
                    }
                    WorldState::GameOver(winner) => {
                        // TODO: THIS IS WRONG!
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
                                ctx.audio.play_sfx_ex(SoundFx::Turn, PlaySoundParams {
                                    looped: false,
                                    volume: 0.5,
                                });
                            } else {
                                // turn SFX
                                // ctx.audio.play_sfx_ex(SoundFx::Turn, PlaySoundParams {
                                //     looped: false,
                                //     volume: 0.5,
                                // });
                            }
                        }
                    }
                }
                WorldEvent::BikeDeath(id, _pos) => {
                    // TODO: spawn explosion effect?
                    if let Some(_player) = self.client.local_player(id) {
                        ctx.audio.play_sfx_vol(SoundFx::Explosion, 1.0);
                    } else {
                        ctx.audio.play_sfx_vol(SoundFx::Explosion, 0.5);
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
                    match self.client.game_state {
                        WorldState::Playing => format!(
                            "{} boost: {} ",
                            player.name,
                            "O".repeat(
                                self.client.grid.bikes
                                    [self.client.server_player(i as u8).unwrap() as usize]
                                    .boost_count as usize
                            )
                        ),
                        _ => format!("{} ready: {} ", player.name, player.ready),
                        // WorldState::RoundOver(_) => todo!(),
                        // WorldState::GameOver(_) => todo!(),
                    },
                    get_color(
                        self.client.grid.bikes
                            [self.client.server_player(i as u8).unwrap_or(0) as usize]
                            .get_color(),
                    ),
                )
            } else if self.client.game_state == WorldState::Waiting {
                ("[A/ENTER/LSHIFT] to join".into(), colors::WHITE)
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
                    a: 0.7,
                },
            );
            let mut color = colors::WHITE;
            let (text, subtext): (String, String) = match self.client.game_state {
                WorldState::GameOver(winner) => (
                    // if let Some(local_player) = self.client.local_player(winner) {
                    {
                        let (team_color, name) = get_team_color(winner);
                        color = team_color;
                        format!("Team {} won!", name)
                    },
                    // } else {
                    //     "Game Lost!".into()
                    // },
                    "Press [enter] to reboot or [delete] to exit.".into(),
                ),
                WorldState::Waiting => (
                    "Game Lobby".into(),
                    "Press [A/ENTER/LSHIFT] to join".into(),
                ),
                WorldState::RoundOver(winner) => (
                    if let Some(winner) = winner {
                        let (team_color, name) = get_team_color(winner);
                        color = team_color;
                        format!("Round won by team {}", name)
                    } else {
                        "Round tied".into()
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

            if matches!(self.client.game_state, WorldState::Waiting) {
                // draw team select
                const TEAM_SPACING: f32 = 200.;
                const PLAYER_SPACING: f32 = 50.;
                pos.x -= (self.client.grid_options.teams - 1) as f32 * TEAM_SPACING / 2.;
                for (i, player) in self.client.server_players.iter().enumerate() {
                    let bike = &self.client.grid.bikes[i];
                    let color = get_color(bike.get_color());
                    let team = bike.team;
                    // let measure = measure_text(&ctx, &player.name, text::Size::Large);
                    // let box_size = text_size(text::Size::Medium);
                    // const SCORE_CENTER_PAD: f32 = 10.;
                    let name = if player.ready {
                        player.name.clone()
                    } else {
                        format!("<{}>", player.name)
                    };
                    draw_text(
                        &ctx,
                        &name,
                        pos.x + TEAM_SPACING * team as f32,
                        pos.y + PLAYER_SPACING * bike.player as f32,
                        text::Size::Large,
                        color,
                    );
                    // pos.y += 50.;
                }
            } else {
                // draw score
                for team in 0..self.client.grid_options.teams {
                    let (color, name) = get_team_color(team);
                    let measure = measure_text(&ctx, &name, text::Size::Large);
                    let box_size = text_size(text::Size::Medium);
                    const SCORE_CENTER_PAD: f32 = 10.;
                    draw_text(
                        &ctx,
                        &name,
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
                        if &score < self.client.scores.get(team as usize).unwrap_or(&0) {
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
}
