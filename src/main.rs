use std::{ops::AddAssign, rc::Rc};
use view::View;
use yewdux::prelude::*;

mod model;
mod view;

use chrono::Duration;
use model::{Cargo, Enemy, Model, Msg, Nationality, Player, ShipClass, MINIMUM_SHIP_HULL};
use rand::seq::SliceRandom;

fn is_cargo_space_available(p: &Player) -> bool {
    p.ship.cargos.total_unit() < p.ship.cargos_capacity
}

fn is_valid_buy(p: &Player, port_cargo: &Cargo) -> bool {
    p.coins > port_cargo.price && is_cargo_space_available(p)
}

impl Reducer<Model> for Msg {
    fn apply(&self, mut model: Rc<Model>) -> Rc<Model> {
        let state = Rc::make_mut(&mut model);

        // TODO: Send alert on insufficient fund or empty cargo unit
        match self {
            Msg::ResetModel => {
                let defmodel = Model::default();
                state.date = defmodel.date;
                state.current_screen = defmodel.current_screen;
                state.current_port_location = defmodel.current_port_location;
                state.player = defmodel.player;
                state.ports = defmodel.ports;
                state.enemy = defmodel.enemy;
            }
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
                            cannons: 4,
                            cannons_capacity: 4,
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
                if state.current_port_location != *l {
                    let days: Vec<i64> = (1..9).collect();
                    state.date.add_assign(Duration::days(
                        *days.choose(&mut rand::thread_rng()).unwrap_or(&1),
                    ));
                    state.current_port_location = *l
                }
            }

            // We don't need to pattern match the get_mut(l)
            // because of enum as hashmap key usage
            Msg::BuyWood(l) => {
                let mut port_cgs = &mut state.ports.get_mut(l).unwrap().cargos;
                if is_valid_buy(&state.player, &port_cgs.wood) {
                    state.player.coins -= &port_cgs.wood.price;
                    port_cgs.wood.unit -= 1;
                    state.player.ship.cargos.wood.unit += 1;
                }
            }
            Msg::BuySugar(l) => {
                let mut port_cgs = &mut state.ports.get_mut(l).unwrap().cargos;
                if is_valid_buy(&state.player, &port_cgs.sugar) {
                    state.player.coins -= port_cgs.sugar.price;
                    port_cgs.sugar.unit -= 1;
                    state.player.ship.cargos.sugar.unit += 1;
                }
            }
            Msg::SellWood(l) => {
                let mut port_wood = &mut state.ports.get_mut(l).unwrap().cargos.wood;
                if state.player.ship.cargos.wood.unit != 0 {
                    state.player.coins += port_wood.price;
                    port_wood.unit += 1;
                    state.player.ship.cargos.wood.unit -= 1;
                }
            }
            Msg::SellSugar(l) => {
                let mut port_sugar = &mut state.ports.get_mut(l).unwrap().cargos.sugar;
                if state.player.ship.cargos.sugar.unit != 0 {
                    state.player.coins += port_sugar.price;
                    port_sugar.unit += 1;
                    state.player.ship.cargos.sugar.unit -= 1;
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
            Msg::SkirmishChaseBroadside => {
                if let Some(enemy) = &mut state.enemy {
                    if enemy.ship.hull < MINIMUM_SHIP_HULL.into() {
                        state.current_screen = model::Screen::MainNavigation;
                    } else {
                        match enemy.distance {
                            model::EnemyDistance::Escape => enemy.ship.hull -= 1,
                            model::EnemyDistance::Far => enemy.ship.hull -= 2,
                            model::EnemyDistance::Close => enemy.ship.hull -= 3,
                            model::EnemyDistance::Board => (),
                        }
                    }
                }
            }
            Msg::SkirmishBattleSwingSword => {
                if let Some(enemy) = &mut state.enemy {
                    enemy.ship.crew -= 1;
                }
            }
            Msg::SkirmishBattleShootFalconet => {
                if let Some(enemy) = &mut state.enemy {
                    enemy.ship.crew -= 2;
                }
            }
            Msg::RepairShip(c) => {
                let defship = model::Model::default().player.ship;
                state.player.ship.hull = defship.hull;
                state.player.ship.cannons = defship.cannons;
            }
        };

        model
    }
}

fn main() {
    yew::Renderer::<View>::new().render();
}
