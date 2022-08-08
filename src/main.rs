use std::{ops::AddAssign, rc::Rc};
use strum::IntoEnumIterator;
use view::View;
use yewdux::prelude::*;

mod model;
mod view;

use chrono::Duration;
use rand::seq::SliceRandom;

fn is_cargo_space_available(p: &model::Player) -> bool {
    p.ship.cargos.total_unit() < p.ship.cargos_capacity
}

fn is_valid_buy(p: &model::Player, port_cargo: &model::Cargo) -> bool {
    p.coins > port_cargo.price && is_cargo_space_available(p)
}

impl Reducer<model::Model> for model::Msg {
    fn apply(&self, mut model: Rc<model::Model>) -> Rc<model::Model> {
        let state = Rc::make_mut(&mut model);

        // TODO: Send alert on insufficient fund or empty cargo unit
        match self {
            model::Msg::ResetModel => {
                let m = model::Model::default();
                state.date = m.date;
                state.current_screen = m.current_screen;
                state.current_port_location = m.current_port_location;
                state.player = m.player;
                state.ports = m.ports;
                state.enemy = m.enemy;
            }
            model::Msg::SwitchScreen(s) => match s {
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

                    let ships: Vec<model::Ship> = vec![
                        model::Ship {
                            class: model::ShipClass::Cutter,
                            crew: 8,
                            crew_capacity: 8,
                            hull: 8,
                            hull_capacity: 8,
                            cannons: 4,
                            cannons_capacity: 8,
                            cargos_capacity: 32,
                            ..Default::default()
                        },
                        model::Ship {
                            class: model::ShipClass::Sloop,
                            crew: 14,
                            crew_capacity: 14,
                            hull: 14,
                            hull_capacity: 14,
                            cannons: 8,
                            cannons_capacity: 8,
                            cargos_capacity: 46,
                            ..Default::default()
                        },
                        model::Ship {
                            class: model::ShipClass::Brig,
                            crew: 18,
                            crew_capacity: 18,
                            hull: 18,
                            hull_capacity: 18,
                            cannons: 10,
                            cannons_capacity: 10,
                            cargos_capacity: 70,
                            ..Default::default()
                        },
                        model::Ship {
                            class: model::ShipClass::Junk,
                            crew: 24,
                            crew_capacity: 18,
                            hull: 18,
                            hull_capacity: 18,
                            cannons: 6,
                            cannons_capacity: 6,
                            cargos_capacity: 80,
                            ..Default::default()
                        },
                        model::Ship {
                            crew: 32,
                            crew_capacity: 32,
                            hull: 27,
                            hull_capacity: 27,
                            cannons: 10,
                            cannons_capacity: 10,
                            cargos_capacity: 210,
                            class: model::ShipClass::Galleon,
                            ..Default::default()
                        },
                        model::Ship {
                            crew: 40,
                            crew_capacity: 40,
                            hull: 35,
                            hull_capacity: 35,
                            cannons: 14,
                            cannons_capacity: 14,
                            cargos_capacity: 150,
                            class: model::ShipClass::Frigate,
                            ..Default::default()
                        },
                    ];

                    let enemy_ship: model::Ship = ships
                        .choose(&mut rand::thread_rng())
                        .unwrap_or(&ships[0])
                        .to_owned();

                    let nationalities: Vec<model::Nationality> =
                        model::Nationality::iter().collect();

                    let new_enemy = model::Enemy {
                        ship: enemy_ship,
                        nationality: nationalities
                            .choose(&mut rand::thread_rng())
                            .unwrap_or(&nationalities[0])
                            .to_owned(),
                        ..Default::default()
                    };
                    state.enemy = Some(new_enemy);
                    state.current_screen = s.to_owned();
                }
                _ => state.current_screen = s.to_owned(),
            },

            model::Msg::SwitchPlayerLocation(l) => {
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
            model::Msg::BuyCargo(l, c) => {
                let mut port_cgs = &mut state.ports.get_mut(l).unwrap().cargos;
                if is_valid_buy(&state.player, &port_cgs.wood) {
                    state.player.coins -= &port_cgs.wood.price;
                    port_cgs.wood.unit -= 1;
                    state.player.ship.cargos.wood.unit += 1;
                }
            }
            model::Msg::SellCargo(l, c) => {
                let mut port_wood = &mut state.ports.get_mut(l).unwrap().cargos.wood;
                if state.player.ship.cargos.wood.unit != 0 {
                    state.player.coins += port_wood.price;
                    port_wood.unit += 1;
                    state.player.ship.cargos.wood.unit -= 1;
                }
            }
            model::Msg::SkirmishChaseClose => {
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
            model::Msg::SkirmishChaseDistant => {
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
            model::Msg::SkirmishChaseBroadside => {
                if let Some(enemy) = &mut state.enemy {
                    if enemy.ship.hull < model::MINIMUM_SHIP_HULL.into() {
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
            model::Msg::SkirmishBattleSwingSword => {
                if let Some(enemy) = &mut state.enemy {
                    enemy.ship.crew -= 1;
                }
            }
            model::Msg::SkirmishBattleShootFalconet => {
                if let Some(enemy) = &mut state.enemy {
                    enemy.ship.crew -= 2;
                }
            }

            // TODO: Subtract with hull and other items cost
            model::Msg::RepairShip(c) => {
                let s = model::Model::default().player.ship;
                state.player.ship.hull = s.hull;
                state.player.ship.cannons = s.cannons;
            }
        };

        model
    }
}

fn main() {
    yew::Renderer::<View>::new().render();
}
