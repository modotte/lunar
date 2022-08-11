use std::collections::HashMap;
use std::fmt::Display;
use yewdux::prelude::*;

use chrono::NaiveDate;
use enum_display_derive::Display;
use serde::{Deserialize, Serialize};

use strum_macros::EnumIter;

pub const MINIMUM_SHIP_HULL: i8 = 2;
pub const MINIMUM_SHIP_CREW: i8 = 2;

#[derive(Default, Clone, PartialEq, Eq, Deserialize, Serialize, Store)]
pub enum Screen {
    MainNavigation,
    About,
    #[default]
    MainMenu,
    NewCharacter,
    Profile,
    Tavern,
    Dock,
    DockMarket,
    DockShipyard,
    Skirmish,
    SkirmishChase,
    SkirmishBattle,
    SkirmishLoot,
}

#[derive(Default, EnumIter, Display, Clone, PartialEq, Eq, Deserialize, Serialize, Store)]
pub enum Nationality {
    #[default]
    British,
    Spanish,
    French,
}

#[derive(Default, EnumIter, Display, Copy, Clone, PartialEq, Eq, Deserialize, Serialize, Store)]
pub enum CargoKind {
    #[default]
    Food,
    Wood,
    Sugar,
}

#[derive(Default, Copy, Clone, PartialEq, Eq, Deserialize, Serialize, Store)]
pub struct Cargo {
    pub price: i32,
    pub unit: i32,
    pub kind: CargoKind,
}

#[derive(Default, Clone, PartialEq, Eq, Deserialize, Serialize, Store)]
pub struct Cargos {
    pub food: Cargo,
    pub wood: Cargo,
    pub sugar: Cargo,
}

impl Cargos {
    pub fn total_unit(&self) -> i32 {
        self.food.unit + self.wood.unit + self.sugar.unit
    }
}

#[derive(Default, Display, Hash, Clone, PartialEq, Eq, Deserialize, Serialize, Store)]
pub enum ShipClass {
    Cutter,
    #[default]
    Sloop,
    Brig,
    Junk,
    Galleon,
    Frigate,
}

#[derive(Default, Clone, PartialEq, Eq, Deserialize, Serialize, Store)]
pub struct Ship {
    pub name: String,
    pub class: ShipClass,
    pub cargos: Cargos,
    pub cargos_capacity: i32,
    pub crew: i32,
    pub crew_capacity: i32,
    pub hull: i16,
    pub hull_capacity: i16,
    pub cannons: i16,
    pub cannons_capacity: i16,
}

#[derive(Default, Clone, PartialEq, Eq, Deserialize, Serialize, Store)]
pub enum EnemyMovement {
    Chase,
    #[default]
    Idle,
    Evade,
}

#[derive(Default, Display, Clone, PartialEq, Eq, Deserialize, Serialize, Store)]
pub enum EnemyDistance {
    Escape,
    #[default]
    Far,
    Close,
    Board,
}

#[derive(Default, Clone, PartialEq, Eq, Deserialize, Serialize, Store)]
pub struct Enemy {
    pub ship: Ship,
    pub nationality: Nationality,
    pub movement: EnemyMovement,
    pub distance: EnemyDistance,
}

#[derive(Default, Clone, PartialEq, Eq, Deserialize, Serialize, Store)]
pub enum PortPopulation {
    Small,
    #[default]
    Medium,
    Large,
    Huge,
}

#[derive(Default, Clone, PartialEq, Eq, Deserialize, Serialize, Store)]
pub struct Port {
    pub name: String,
    pub description: String,
    pub population: PortPopulation,
    pub nationality: Nationality,
    pub cargos: Cargos,
}

#[derive(Default, Clone, PartialEq, Eq, Deserialize, Serialize, Store)]
pub struct Player {
    pub name: String,
    pub age: i8,
    pub nationality: Nationality,
    pub coins: i32,
    pub ship: Ship,
}

#[derive(Default, Display, Copy, Hash, Clone, PartialEq, Eq, Deserialize, Serialize, Store)]
pub enum PortLocation {
    Barbados,
    #[default]
    PortRoyal,
    Nassau,
}

pub type Ports = HashMap<PortLocation, Port>;

#[derive(Clone, PartialEq, Eq, Deserialize, Serialize, Store)]
#[store(storage = "local")]
pub struct Model {
    pub date: NaiveDate,
    pub current_screen: Screen,
    pub player: Player,
    pub ports: Ports,
    pub current_port_location: PortLocation,
    pub enemy: Option<Enemy>,
}

// Initializer for our whole model at launch
impl Default for Model {
    fn default() -> Self {
        Self {
            date: NaiveDate::from_ymd(1680, 01, 01),
            player: Player {
                name: String::from("Player"),
                age: 18,
                nationality: Nationality::British,
                coins: 1000,
                ship: Ship {
                    name: String::from("Luna"),
                    crew: 12,
                    cargos_capacity: 32,
                    hull: 40,
                    hull_capacity: 40,
                    cannons: 10,
                    cannons_capacity: 10,
                    ..Default::default()
                },
            },
            ports: HashMap::from([
                (
                    PortLocation::Barbados,
                    Port {
                        name: String::from("Barbados"),
                        description: String::default(),
                        population: PortPopulation::Large,
                        nationality: Nationality::British,
                        cargos: Cargos {
                            food: Cargo {
                                price: 8,
                                unit: 250,
                                kind: CargoKind::Food,
                            },
                            wood: Cargo {
                                price: 22,
                                unit: 250,
                                kind: CargoKind::Wood,
                            },
                            sugar: Cargo {
                                price: 30,
                                unit: 250,
                                kind: CargoKind::Sugar,
                            },
                        },
                    },
                ),
                (
                    PortLocation::PortRoyal,
                    Port {
                        name: String::from("Port Royal"),
                        description: String::default(),
                        population: PortPopulation::Huge,
                        nationality: Nationality::British,
                        cargos: Cargos {
                            food: Cargo {
                                price: 5,
                                unit: 250,
                                kind: CargoKind::Food,
                            },
                            wood: Cargo {
                                price: 18,
                                unit: 210,
                                kind: CargoKind::Wood,
                            },
                            sugar: Cargo {
                                price: 50,
                                unit: 120,
                                kind: CargoKind::Sugar,
                            },
                        },
                    },
                ),
                (
                    PortLocation::Nassau,
                    Port {
                        name: String::from("Nassau"),
                        description: String::default(),
                        population: PortPopulation::Medium,
                        nationality: Nationality::British,
                        cargos: Cargos {
                            food: Cargo {
                                price: 10,
                                unit: 250,
                                kind: CargoKind::Food,
                            },
                            wood: Cargo {
                                price: 32,
                                unit: 150,
                                kind: CargoKind::Wood,
                            },
                            sugar: Cargo {
                                price: 20,
                                unit: 180,
                                kind: CargoKind::Sugar,
                            },
                        },
                    },
                ),
            ]),
            current_screen: Screen::default(),
            current_port_location: PortLocation::default(),
            enemy: None,
        }
    }
}

pub enum Msg {
    // TODO: Remove ResetModel when debug isn't needed anymore!!!
    ResetModel,
    SwitchScreen(Screen),
    SwitchPlayerLocation(PortLocation),
    BuyCargo(PortLocation, Cargo),
    SellCargo(PortLocation, Cargo),
    SkirmishChaseClose,
    SkirmishChaseDistant,
    SkirmishChaseBroadside,
    SkirmishBattleSwingSword,
    SkirmishBattleShootFalconet,
    RepairShip(i32),
}
