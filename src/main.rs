use std::{ops::AddAssign, rc::Rc};
use view::View;
use yewdux::prelude::*;

mod model;
mod view;

use chrono::Duration;
use model::{Cargo, Enemy, Model, Msg, Nationality, Player, ShipClass};
use rand::seq::SliceRandom;

impl Reducer<Model> for Msg {
    fn apply(&self, mut model: Rc<Model>) -> Rc<Model> {
        let state = Rc::make_mut(&mut model);
        let is_cargo_space_available =
            |p: &Player| p.ship.cargo.total_unit() < p.ship.cargo_capacity;
        let is_valid_buy = |player: &Player, port_cargo: &Cargo| {
            player.coins > port_cargo.price && is_cargo_space_available(player)
        };

        // TODO: Send alert on insufficient fund or empty cargo unit
        match self {
            Msg::SwitchScreen(s) => match s {
                model::Screen::MainNavigation => {
                    state.enemy = None;
                    state.current_screen = s.to_owned()
                }
                model::Screen::Skirmish => {
                    let names = vec![
                        "Shady Wave",
                        "Palm West",
                        "Southern Seas",
                        "Morning Star",
                        "Blue Ocean",
                    ];
                    let new_enemy = Enemy {
                        ship: model::Ship {
                            name: names
                                .choose(&mut rand::thread_rng())
                                .unwrap_or(&names[0])
                                .to_string(),
                            class: ShipClass::Sloop,
                            nationality: Nationality::British,
                            crew: 7,
                            crew_capacity: 8,
                            hull: 8,
                            hull_capacity: 8,
                            cannon: 4,
                            cannon_capacity: 4,
                            ..Default::default()
                        },
                        ..Default::default()
                    };
                    state.enemy = Some(new_enemy);
                    state.current_screen = s.to_owned();
                }
                _ => state.current_screen = s.to_owned(),
            },

            Msg::SwitchPlayerLocation(l) => {
                if state.current_location != *l {
                    let days: Vec<i64> = (1..9).collect();
                    state.date.add_assign(Duration::days(
                        *days.choose(&mut rand::thread_rng()).unwrap_or(&1),
                    ));
                    state.current_location = *l
                }
            }

            // We don't need to pattern match the get_mut(l)
            // because of enum as hashmap key usage
            Msg::BuyWood(l) => {
                let mut port_cgi = &mut state.locations.get_mut(l).unwrap().cargo;
                if is_valid_buy(&state.player, &port_cgi.wood) {
                    state.player.coins -= port_cgi.wood.price;
                    port_cgi.wood.unit -= 1;
                    state.player.ship.cargo.wood.unit += 1;
                }
            }
            Msg::BuySugar(l) => {
                let mut port_cgi = &mut state.locations.get_mut(l).unwrap().cargo;
                if is_valid_buy(&state.player, &port_cgi.sugar) {
                    state.player.coins -= port_cgi.wood.price;
                    port_cgi.wood.unit -= 1;
                    state.player.ship.cargo.sugar.unit += 1;
                }
            }
            Msg::SellWood(l) => {
                let mut port_wood = &mut state.locations.get_mut(l).unwrap().cargo.wood;
                if state.player.ship.cargo.wood.unit != 0 {
                    state.player.coins += port_wood.price;
                    port_wood.unit += 1;
                    state.player.ship.cargo.wood.unit -= 1;
                }
            }
            Msg::SellSugar(l) => {
                let mut port_sugar = &mut state.locations.get_mut(l).unwrap().cargo.sugar;
                if state.player.ship.cargo.sugar.unit != 0 {
                    state.player.coins += port_sugar.price;
                    port_sugar.unit += 1;
                    state.player.ship.cargo.sugar.unit -= 1;
                }
            }
            Msg::SkirmishChaseClose => {
                if let Some(enemy) = &mut state.enemy {
                    match enemy.distance {
                        model::EnemyDistance::Escape => enemy.distance = model::EnemyDistance::Far,
                        model::EnemyDistance::Far => enemy.distance = model::EnemyDistance::Close,
                        model::EnemyDistance::Close => enemy.distance = model::EnemyDistance::Board,
                        model::EnemyDistance::Board => {
                            state.current_screen = model::Screen::SkirmishBattle
                        }
                    }
                }
            }
            Msg::SkirmishChaseDistant => {
                if let Some(enemy) = &mut state.enemy {
                    match enemy.distance {
                        model::EnemyDistance::Escape => {
                            state.enemy = None;
                            state.current_screen = model::Screen::MainNavigation
                        }
                        model::EnemyDistance::Far => enemy.distance = model::EnemyDistance::Escape,
                        model::EnemyDistance::Close => enemy.distance = model::EnemyDistance::Far,
                        model::EnemyDistance::Board => {
                            state.current_screen = model::Screen::SkirmishBattle
                        }
                    }
                }
            }
        };

        model
    }
}

fn main() {
    yew::Renderer::<View>::new().render();
}
