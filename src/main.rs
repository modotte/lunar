use std::{collections::HashMap, ops::AddAssign, rc::Rc};
use strum::IntoEnumIterator;
use view::View;
use yewdux::prelude::*;

mod model;
mod view;

use chrono::Duration;
use rand::{seq::SliceRandom, Rng};

fn is_cargo_space_available(p: &model::Player) -> bool {
    p.ship.cargos.total_unit() < p.ship.cargos_capacity
}

fn is_valid_buy(p: &model::Player, port_cargo: &model::Cargo) -> bool {
    p.coins > port_cargo.price && is_cargo_space_available(p)
}

fn choice_of<T: Clone>(sequence: &[T], default: &T) -> T {
    sequence
        .choose(&mut rand::thread_rng())
        .unwrap_or(default)
        .to_owned()
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

                    let enemy_ship = choice_of(&model::SHIPS, &model::SHIPS[0]);

                    let nationalities: Vec<model::Nationality> =
                        model::Nationality::iter().collect();

                    let mut new_enemy = model::Enemy {
                        ship: enemy_ship,
                        nationality: choice_of(&nationalities, &nationalities[0]),
                        ..Default::default()
                    };

                    new_enemy.ship.name = choice_of(&names, &names[0]).to_string();
                    let mut rng = rand::thread_rng();
                    new_enemy.ship.cargos.food.unit = rng.gen_range(1..=8);
                    new_enemy.ship.cargos.wood.unit = rng.gen_range(1..=8);
                    new_enemy.ship.cargos.sugar.unit = rng.gen_range(1..=10);

                    state.enemy = Some(new_enemy);
                    state.current_screen = s.to_owned();
                }
                _ => state.current_screen = s.to_owned(),
            },

            model::Msg::SwitchPlayerLocation(l) => {
                if state.current_port_location != *l {
                    let days: i64 = rand::thread_rng().gen_range(1..=9);
                    state.date.add_assign(Duration::days(days));
                    state.current_port_location = *l;

                    let f = |mut p: model::Port| -> model::Port {
                        let mut rng = rand::thread_rng();
                        let cargos = &mut p.cargos;

                        cargos.wood.unit = rng.gen_range(200..=250);
                        cargos.sugar.unit = rng.gen_range(120..=200);

                        cargos.wood.price = rng.gen_range(18..=60);
                        cargos.sugar.price = rng.gen_range(18..=90);

                        p
                    };

                    state.ports = state
                        .ports
                        .clone()
                        .into_iter()
                        .map(|(k, v)| (k, f(v)))
                        .collect::<HashMap<model::PortLocation, model::Port>>();

                    let player_food = &mut state.player.ship.cargos.food.unit;

                    if *player_food < model::MINIMUM_PLAYER_FOOD.into() {
                        state.current_screen =
                            model::Screen::GameLost(model::GameLostReason::PlayerFoodMutiny);
                        let m = model::Model::default();
                        state.date = m.date;
                        state.current_port_location = m.current_port_location;
                        state.player = m.player;
                        state.ports = m.ports;
                    } else {
                        *player_food -= rand::thread_rng().gen_range(0..=1);
                    }
                }
            }

            // We don't need to pattern match the get_mut(l)
            // because of enum as hashmap key usage
            model::Msg::BuyCargo(l, port_cargo) => {
                let mut port_cgs = &mut state.ports.get_mut(l).unwrap().cargos;
                if is_valid_buy(&state.player, port_cargo) {
                    state.player.coins -= port_cargo.price;
                    match port_cargo.kind {
                        model::CargoKind::Food => {
                            port_cgs.food.unit -= 1;
                            state.player.ship.cargos.food.unit += 1;
                        }
                        model::CargoKind::Wood => {
                            port_cgs.wood.unit -= 1;
                            state.player.ship.cargos.wood.unit += 1;
                        }
                        model::CargoKind::Sugar => {
                            port_cgs.sugar.unit -= 1;
                            state.player.ship.cargos.sugar.unit += 1;
                        }
                    }
                }
            }
            model::Msg::SellCargo(l, port_cargo) => {
                let mut ports_cgs = &mut state.ports.get_mut(l).unwrap().cargos;

                match port_cargo.kind {
                    model::CargoKind::Food => {
                        if state.player.ship.cargos.food.unit != 0 {
                            state.player.coins += port_cargo.price;
                            ports_cgs.food.unit += 1;
                            state.player.ship.cargos.food.unit -= 1;
                        }
                    }
                    model::CargoKind::Wood => {
                        if state.player.ship.cargos.wood.unit != 0 {
                            state.player.coins += port_cargo.price;
                            ports_cgs.wood.unit += 1;
                            state.player.ship.cargos.wood.unit -= 1;
                        }
                    }
                    model::CargoKind::Sugar => {
                        if state.player.ship.cargos.sugar.unit != 0 {
                            state.player.coins += port_cargo.price;
                            ports_cgs.sugar.unit += 1;
                            state.player.ship.cargos.sugar.unit -= 1;
                        }
                    }
                }
            }
            model::Msg::SkirmishChaseClose => {
                if let Some(enemy) = &mut state.enemy {
                    if state.player.ship.hull < model::MINIMUM_SHIP_HULL.into() {
                        state.current_screen =
                            model::Screen::GameLost(model::GameLostReason::PlayerShipSunk);
                        let m = model::Model::default();
                        state.date = m.date;
                        state.current_port_location = m.current_port_location;
                        state.player = m.player;
                        state.ports = m.ports;
                    }

                    if enemy.ship.hull < model::MINIMUM_SHIP_HULL.into() {
                        state.current_screen = model::Screen::MainNavigation;
                    } else {
                        let mut rng = rand::thread_rng();
                        let player_cannons = state.player.ship.cannons;
                        let enemy_cannons = enemy.ship.cannons;
                        match enemy.distance {
                            model::EnemyDistance::Escape => {
                                state.player.ship.hull -= rng.gen_range(0..=enemy_cannons - 3);

                                enemy.ship.hull -= rng.gen_range(0..=player_cannons - 3);
                                enemy.distance = model::EnemyDistance::Far
                            }
                            model::EnemyDistance::Far => {
                                state.player.ship.hull -= rng.gen_range(0..=enemy_cannons - 3);

                                enemy.ship.hull -= rng.gen_range(0..=player_cannons - 3);
                                enemy.distance = model::EnemyDistance::Close
                            }
                            model::EnemyDistance::Close => {
                                state.player.ship.hull -= rng.gen_range(1..=enemy_cannons - 2);

                                enemy.ship.hull -= rng.gen_range(1..=player_cannons - 2);
                                enemy.distance = model::EnemyDistance::Board
                            }
                            model::EnemyDistance::Board => {
                                state.current_screen = model::Screen::SkirmishBattle
                            }
                        }
                    }
                }
            }
            model::Msg::SkirmishChaseDistant => {
                if let Some(enemy) = &mut state.enemy {
                    if state.player.ship.hull < model::MINIMUM_SHIP_HULL.into() {
                        state.current_screen =
                            model::Screen::GameLost(model::GameLostReason::PlayerShipSunk);

                        let m = model::Model::default();
                        state.date = m.date;
                        state.current_port_location = m.current_port_location;
                        state.player = m.player;
                        state.ports = m.ports;
                    }

                    if enemy.ship.hull < model::MINIMUM_SHIP_HULL.into() {
                        state.current_screen = model::Screen::MainNavigation;
                    } else {
                        let mut rng = rand::thread_rng();
                        let player_cannons = state.player.ship.cannons;
                        let enemy_cannons = enemy.ship.cannons;
                        match enemy.distance {
                            model::EnemyDistance::Escape => {
                                state.enemy = None;
                                state.current_screen = model::Screen::MainNavigation
                            }
                            model::EnemyDistance::Far => {
                                state.player.ship.hull -= rng.gen_range(0..=enemy_cannons - 3);

                                enemy.ship.hull -= rng.gen_range(0..=player_cannons - 3);
                                enemy.distance = model::EnemyDistance::Escape
                            }
                            model::EnemyDistance::Close => {
                                state.player.ship.hull -= rng.gen_range(0..=enemy_cannons - 2);

                                enemy.ship.hull -= rng.gen_range(0..=player_cannons - 2);
                                enemy.distance = model::EnemyDistance::Far
                            }
                            model::EnemyDistance::Board => {
                                state.current_screen = model::Screen::SkirmishBattle
                            }
                        }
                    }
                }
            }
            model::Msg::SkirmishChaseBroadside => {
                if let Some(enemy) = &mut state.enemy {
                    if state.player.ship.hull < model::MINIMUM_SHIP_HULL.into() {
                        state.current_screen =
                            model::Screen::GameLost(model::GameLostReason::PlayerShipSunk);

                        let m = model::Model::default();
                        state.date = m.date;
                        state.current_port_location = m.current_port_location;
                        state.player = m.player;
                        state.ports = m.ports;
                    }

                    if enemy.ship.hull < model::MINIMUM_SHIP_HULL.into() {
                        state.current_screen = model::Screen::MainNavigation;
                    } else {
                        let mut rng = rand::thread_rng();
                        let player_cannons = state.player.ship.cannons;
                        let enemy_cannons = enemy.ship.cannons;
                        match enemy.distance {
                            model::EnemyDistance::Escape => {
                                state.player.ship.hull -= rng.gen_range(0..=enemy_cannons - 4);

                                enemy.ship.hull -= rng.gen_range(0..=player_cannons - 4);
                            }
                            model::EnemyDistance::Far => {
                                state.player.ship.hull -= rng.gen_range(0..=enemy_cannons - 3);
                                enemy.ship.hull -= rng.gen_range(0..=player_cannons - 3);
                            }
                            model::EnemyDistance::Close => {
                                state.player.ship.hull -= rng.gen_range(1..=enemy_cannons - 2);

                                enemy.ship.hull -= rng.gen_range(1..=player_cannons - 2);
                            }
                            model::EnemyDistance::Board => (),
                        }
                    }
                }
            }
            model::Msg::SkirmishBattleSwingSword => {
                if let Some(enemy) = &mut state.enemy {
                    if state.player.ship.crew < model::MINIMUM_SHIP_CREW.into() {
                        state.current_screen =
                            model::Screen::GameLost(model::GameLostReason::PlayerAllCrewDied);

                        let m = model::Model::default();
                        state.date = m.date;
                        state.current_port_location = m.current_port_location;
                        state.player = m.player;
                        state.ports = m.ports;
                    }
                    state.player.ship.crew -= rand::thread_rng().gen_range(1..=2);

                    enemy.ship.crew -= rand::thread_rng().gen_range(1..=2);
                }
            }
            model::Msg::SkirmishBattleShootFalconet => {
                if let Some(enemy) = &mut state.enemy {
                    if state.player.ship.crew < model::MINIMUM_SHIP_CREW.into() {
                        state.current_screen =
                            model::Screen::GameLost(model::GameLostReason::PlayerAllCrewDied);
                        let m = model::Model::default();
                        state.date = m.date;
                        state.current_port_location = m.current_port_location;
                        state.player = m.player;
                        state.ports = m.ports;
                    }
                    state.player.ship.crew -= rand::thread_rng().gen_range(1..=4);

                    enemy.ship.crew -= rand::thread_rng().gen_range(1..=5);
                }
            }

            // TODO: Subtract with hull and other items cost
            model::Msg::RepairShip(c) => {
                let s = model::Model::default().player.ship;
                state.player.ship.hull = s.hull;
            }

            model::Msg::TakeEnemyCargo(c) => {
                if let Some(enemy) = &mut state.enemy {
                    if enemy.ship.cargos.total_unit() > 0 {
                        let enemy_cargos = &mut enemy.ship.cargos;
                        match c {
                            model::CargoKind::Food => {
                                enemy_cargos.food.unit -= 1;
                                state.player.ship.cargos.food.unit += 1;
                            }
                            model::CargoKind::Wood => {
                                enemy_cargos.wood.unit -= 1;
                                state.player.ship.cargos.wood.unit += 1;
                            }
                            model::CargoKind::Sugar => {
                                enemy_cargos.sugar.unit -= 1;
                                state.player.ship.cargos.sugar.unit += 1;
                            }
                        }
                    }
                }
            }
            model::Msg::BuyAndReplaceShip(sc) => match sc {
                model::ShipClass::Cutter => {
                    let s = model::SHIPS[0].clone();
                    if state.player.coins >= s.price {
                        state.player.ship = s;
                    }
                }
                model::ShipClass::Sloop => {
                    let s = model::SHIPS[1].clone();
                    if state.player.coins >= s.price {
                        state.player.ship = s;
                    }
                }
                model::ShipClass::Brig => {
                    let s = model::SHIPS[2].clone();
                    if state.player.coins >= s.price {
                        state.player.ship = s;
                    }
                }
                model::ShipClass::Junk => {
                    let s = model::SHIPS[3].clone();
                    if state.player.coins >= s.price {
                        state.player.ship = s;
                    }
                }
                model::ShipClass::Galleon => {
                    let s = model::SHIPS[4].clone();
                    if state.player.coins >= s.price {
                        state.player.ship = s;
                    }
                }
                model::ShipClass::Frigate => {
                    let s = model::SHIPS[5].clone();
                    if state.player.coins >= s.price {
                        state.player.ship = s;
                    }
                }
            },
        };

        model
    }
}

fn main() {
    yew::Renderer::<View>::new().render();
}
