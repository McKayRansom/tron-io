use std::collections::HashMap;

use macroquad::audio::PlaySoundParams;
use macroquad::color::colors;
use macroquad::prelude::*;
use tron_io_world::client::{WorldClient, WorldEvent};
use tron_io_world::grid::bike::{FLAG_BOOST, FLAG_SHOOT};
use tron_io_world::local::WorldClientLocal;
use tron_io_world::{GridOptions, WorldState};

use crate::colors::get_team_color;
use crate::context::{Context, GRID_VIEWPORT_SIZE, VIRTUAL_HEIGHT, VIRTUAL_WIDTH};
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
                    if bike_update.flags & FLAG_SHOOT != 0 {
                        ctx.audio.play_sfx(SoundFx::Turn);
                    }
                    if bike_update.flags & FLAG_BOOST != 0 {
                        ctx.audio.play_sfx(SoundFx::Boost);
                    }
                    // THIS IS SUPER ANNOYING
                    // turn SFX
                    // ctx.audio.play_sfx(SoundFx::Turn);
                    // }
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
                            if update.flags & FLAG_SHOOT != 0 {
                                ctx.audio.play_sfx_ex(SoundFx::Turn, PlaySoundParams {
                                    looped: false,
                                    volume: 0.5,
                                });
                            }
                            // if update.flags & FLAG_BOOST == 0 {
                            //     ctx.audio.play_sfx_ex(SoundFx::Boost, PlaySoundParams {
                            //         looped: false,
                            //         volume: 0.5,
                            //     });
                            // }
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
        draw_grid(&self.client.grid, ctx);

        // draw players in corners
        for i in 0..4 {
            const PLAYER_EDGE_SPACING: f32 = 24.;
            let pos = match i {
                0 => vec2(PLAYER_EDGE_SPACING, PLAYER_EDGE_SPACING),
                1 => vec2(VIRTUAL_WIDTH - PLAYER_EDGE_SPACING, PLAYER_EDGE_SPACING),
                2 => vec2(PLAYER_EDGE_SPACING, VIRTUAL_HEIGHT - PLAYER_EDGE_SPACING),
                3 => vec2(
                    VIRTUAL_WIDTH - PLAYER_EDGE_SPACING,
                    VIRTUAL_HEIGHT - PLAYER_EDGE_SPACING,
                ),
                _ => unreachable!(),
            };
            let (mut text_val, mut color, mut alive) =
                ("[A/ENTER/LSHIFT] to join".to_string(), colors::WHITE, false);
            if let Some(player) = self.client.local_players.get(i) {
                let bike = &self.client.grid.bikes
                    [self.client.server_player(i as u8).unwrap_or(0) as usize];
                text_val = match self.client.game_state {
                    WorldState::Playing => format!(
                        "{} boost: {} gun: {}",
                        player.name,
                        "O".repeat(bike.boost_count as usize),
                        "O".repeat(bike.bullet_count as usize)
                    ),
                    _ => format!(
                        "{} {} ",
                        player.name,
                        if player.ready {
                            "READY"
                        } else {
                            "Press confirm to ready up"
                        }
                    ),
                };
                color = get_color(bike.get_color());
                alive = bike.alive;
            } else if self.client.game_state != WorldState::Waiting {
                continue;
            }

            let measure = measure_text(ctx, &text_val, text::Size::Small);
            draw_text_ex(
                &text_val,
                if i % 2 == 0 {
                    pos.x
                } else {
                    pos.x - measure.width
                },
                pos.y,
                TextParams {
                    font: if alive {
                        Some(&ctx.font)
                    } else {
                        Some(&ctx.font_line)
                    },
                    color: color,
                    font_size: text_size(text::Size::Small),
                    ..Default::default()
                },
            );
            if let Some((input_type, _player)) =
                self.input_map.iter().find(|item| *item.1 == i as u8)
            {
                let text = input_type.help();
                let measure = measure_text(ctx, &text, text::Size::Small);
                draw_text_ex(
                    &text,
                    if i % 2 == 0 {
                        pos.x 
                    } else {
                        pos.x - measure.width
                    } + 10.,
                    pos.y + measure.height,
                    TextParams {
                        font: if alive {
                            Some(&ctx.font)
                        } else {
                            Some(&ctx.font_line)
                        },
                        color: color,
                        font_size: text_size(text::Size::Small),
                        ..Default::default()
                    },
                );
            }
        }

        // draw game over
        if matches!(
            self.client.game_state,
            WorldState::GameOver(_) | WorldState::Waiting | WorldState::RoundOver(_)
        ) {
            let size: f32 = GRID_VIEWPORT_SIZE;
            draw_rectangle(
                (VIRTUAL_WIDTH - size) / 2.,
                (VIRTUAL_HEIGHT - size) / 2.,
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
                WorldState::Waiting => {
                    ("Game Lobby".into(), "Press [A/ENTER/LSHIFT] to join".into())
                }
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

            let mut pos: Vec2 = vec2(VIRTUAL_WIDTH / 2., 200.);

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
