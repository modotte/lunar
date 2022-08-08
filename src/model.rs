use std::collections::HashMap;
use std::fmt::Display;
use yewdux::prelude::*;

use chrono::NaiveDate;
use enum_display_derive::Display;
use serde::{Deserialize, Serialize};

pub const MINIMUM_SHIP_HULL: u8 = 2;
pub const MINIMUM_SHIP_CREW: u8 = 2;

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
    Skirmish,
    SkirmishChase,
    SkirmishBattle,
    SkirmishLoot,
}

#[derive(Default, Display, Clone, PartialEq, Eq, Deserialize, Serialize, Store)]
pub enum Nationality {
    #[default]
    British,
    Spanish,
    French,
}

#[derive(Default, Clone, PartialEq, Eq, Deserialize, Serialize, Store)]
pub struct Cargo {
    pub name: String,
    pub description: String,
    pub price: u32,
    pub unit: u32,
}

#[derive(Default, Clone, PartialEq, Eq, Deserialize, Serialize, Store)]
pub struct CargoItems {
    pub wood: Cargo,
    pub sugar: Cargo,
}

impl CargoItems {
    pub fn total_unit(&self) -> u32 {
        self.wood.unit + self.sugar.unit
    }
}

#[derive(Default, Clone, PartialEq, Eq, Deserialize, Serialize, Store)]
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
    pub nationality: Nationality,
    pub cargo: CargoItems,
    pub cargo_capacity: u32,
    pub crew: u32,
    pub crew_capacity: u32,
    pub hull: u16,
    pub hull_capacity: u16,
    pub cannon: u16,
    pub cannon_capacity: u16,
}

#[derive(Default, Clone, PartialEq, Eq, Deserialize, Serialize, Store)]
pub enum EnemyMovement {
    Chase,
    #[default]
    Idle,
    Evade,
}

#[derive(Default, Clone, PartialEq, Eq, Deserialize, Serialize, Store)]
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
    pub cargo: CargoItems,
}

#[derive(Default, Clone, PartialEq, Eq, Deserialize, Serialize, Store)]
pub struct Player {
    pub name: String,
    pub age: u8,
    pub coins: u32,
    pub ship: Ship,
}

#[derive(Default, Display, Copy, Hash, Clone, PartialEq, Eq, Deserialize, Serialize, Store)]
pub enum PortLocation {
    Barbados,
    #[default]
    PortRoyal,
    Nassau,
}

#[derive(Clone, PartialEq, Eq, Deserialize, Serialize, Store)]
#[store(storage = "local")]
pub struct Model {
    pub date: NaiveDate,
    pub current_screen: Screen,
    pub player: Player,
    pub ports: HashMap<PortLocation, Port>,
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
                coins: 1000,
                ship: Ship {
                    name: String::from("Luna"),
                    crew: 12,
                    cargo_capacity: 32,
                    hull: 10,
                    hull_capacity: 10,
                    cannon: 4,
                    cannon_capacity: 4,
                    ..Default::default()
                },
            },
            ports: HashMap::from([
                (
                    PortLocation::Barbados,
                    Port {
                        name: String::from("Barbados"),
                        description: String::from(""),
                        population: PortPopulation::Large,
                        nationality: Nationality::British,
                        cargo: CargoItems {
                            wood: Cargo {
                                name: String::from("Wood"),
                                description: String::from(""),
                                price: 22,
                                unit: 250,
                            },
                            sugar: Cargo {
                                name: String::from("Sugar"),
                                description: String::from(""),
                                price: 30,
                                unit: 250,
                            },
                        },
                    },
                ),
                (
                    PortLocation::PortRoyal,
                    Port {
                        name: String::from("Port Royal"),
                        description: String::from(""),
                        population: PortPopulation::Huge,
                        nationality: Nationality::British,
                        cargo: CargoItems {
                            wood: Cargo {
                                name: String::from("Wood"),
                                description: String::from(""),
                                price: 18,
                                unit: 500,
                            },
                            sugar: Cargo {
                                name: String::from("Sugar"),
                                description: String::from(""),
                                price: 50,
                                unit: 120,
                            },
                        },
                    },
                ),
                (
                    PortLocation::Nassau,
                    Port {
                        name: String::from("Nassau"),
                        description: String::from(""),
                        population: PortPopulation::Medium,
                        nationality: Nationality::British,
                        cargo: CargoItems {
                            wood: Cargo {
                                name: String::from("Wood"),
                                description: String::from(""),
                                price: 32,
                                unit: 150,
                            },
                            sugar: Cargo {
                                name: String::from("Sugar"),
                                description: String::from(""),
                                price: 20,
                                unit: 180,
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
    // TODO: Remove ResetData when debug isn't needed anymore!!!
    ResetModel,
    SwitchScreen(Screen),
    SwitchPlayerLocation(PortLocation),
    BuyWood(PortLocation),
    BuySugar(PortLocation),
    SellWood(PortLocation),
    SellSugar(PortLocation),
    SkirmishChaseClose,
    SkirmishChaseDistant,
    SkirmishChaseBroadside,
    SkirmishBattleSwingSword,
    SkirmishBattleShootFalconet,
}
