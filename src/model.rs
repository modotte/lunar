use std::fmt::Display;
use std::string::ParseError;
use std::{collections::HashMap, str::FromStr};
use strum::IntoEnumIterator;
use yewdux::prelude::*;

use chrono::NaiveDate;
use enum_display_derive::Display;
use serde::{Deserialize, Serialize};

use lazy_static::lazy_static;
use strum_macros::EnumIter;

pub const MINIMUM_PLAYER_FOOD: i8 = 4;
pub const MINIMUM_SHIP_HULL: i8 = 2;
pub const MINIMUM_SHIP_CREW: i8 = 2;
pub const MINIMUM_PLAYER_AGE: i8 = 18;
pub const MAXIMUM_PLAYER_AGE: i8 = 65;

lazy_static! {
    pub static ref SHIPS: HashMap<ShipClass, Ship> = HashMap::from([
        (
            ShipClass::Cutter,
            Ship {
                class: ShipClass::Cutter,
                crew: 8,
                crew_capacity: 8,
                hull: 40,
                hull_capacity: 40,
                cannons: 8,
                cannons_capacity: 8,
                cargos_capacity: 32,
                price: 2100,
                cargos: Cargos {
                    food: Cargo {
                        unit: 12,
                        kind: CargoKind::Food,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                ..Default::default()
            }
        ),
        (
            ShipClass::Sloop,
            Ship {
                class: ShipClass::Sloop,
                crew: 14,
                crew_capacity: 14,
                hull: 62,
                hull_capacity: 62,
                cannons: 8,
                cannons_capacity: 8,
                cargos_capacity: 46,
                price: 3200,
                cargos: Cargos {
                    food: Cargo {
                        unit: 19,
                        kind: CargoKind::Food,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                ..Default::default()
            }
        ),
        (
            ShipClass::Brig,
            Ship {
                class: ShipClass::Brig,
                crew: 18,
                crew_capacity: 18,
                hull: 70,
                hull_capacity: 70,
                cannons: 10,
                cannons_capacity: 10,
                cargos_capacity: 70,
                price: 5000,
                cargos: Cargos {
                    food: Cargo {
                        unit: 32,
                        kind: CargoKind::Food,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                ..Default::default()
            }
        ),
        (
            ShipClass::Junk,
            Ship {
                class: ShipClass::Junk,
                crew: 24,
                crew_capacity: 24,
                hull: 70,
                hull_capacity: 70,
                cannons: 6,
                cannons_capacity: 6,
                cargos_capacity: 80,
                price: 5500,
                cargos: Cargos {
                    food: Cargo {
                        unit: 36,
                        kind: CargoKind::Food,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                ..Default::default()
            }
        ),
        (
            ShipClass::Galleon,
            Ship {
                crew: 32,
                crew_capacity: 32,
                hull: 90,
                hull_capacity: 90,
                cannons: 10,
                cannons_capacity: 10,
                cargos_capacity: 210,
                class: ShipClass::Galleon,
                price: 10_000,
                cargos: Cargos {
                    food: Cargo {
                        unit: 60,
                        kind: CargoKind::Food,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                ..Default::default()
            }
        ),
        (
            ShipClass::Frigate,
            Ship {
                crew: 40,
                crew_capacity: 40,
                hull: 140,
                hull_capacity: 140,
                cannons: 14,
                cannons_capacity: 14,
                cargos_capacity: 150,
                class: ShipClass::Frigate,
                price: 35_000,
                cargos: Cargos {
                    food: Cargo {
                        unit: 42,
                        kind: CargoKind::Food,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                ..Default::default()
            }
        ),
    ]);
    pub static ref NATIONALITIES: Vec<Nationality> = Nationality::iter().collect();
    pub static ref SHIP_CLASSES: Vec<ShipClass> = ShipClass::iter().collect();
}

#[derive(Default, Clone, PartialEq, Eq, Deserialize, Serialize, Store)]
pub enum Screen {
    MainNavigation,
    About,
    #[default]
    MainMenu,
    NewCharacter,
    Profile,
    Dock,
    DockTavern,
    TavernHireCrew,
    DockMarket,
    DockShipyard,
    Skirmish,
    SkirmishChase,
    SkirmishBattle,
    SkirmishLoot,
    GameLost(GameLostReason),
}

#[derive(Default, Copy, Clone, PartialEq, Eq, Deserialize, Serialize, Store)]
pub enum GameLostReason {
    #[default]
    ShipSunk,
    AllCrewDied,
    FoodMutiny,
}

#[derive(Default, EnumIter, Display, Clone, PartialEq, Eq, Deserialize, Serialize, Store)]
pub enum Nationality {
    #[default]
    British,
    Spanish,
    French,
}

impl FromStr for Nationality {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Spanish" => Ok(Self::Spanish),
            "French" => Ok(Self::French),
            _otherwise => Ok(Self::British),
        }
    }
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

#[derive(
    Default, Copy, Display, EnumIter, Hash, Clone, PartialEq, Eq, Deserialize, Serialize, Store,
)]
pub enum ShipClass {
    Cutter,
    #[default]
    Sloop,
    Brig,
    Junk,
    Galleon,
    Frigate,
}

impl FromStr for ShipClass {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Cutter" => Ok(Self::Cutter),
            "Brig" => Ok(Self::Brig),
            "Junk" => Ok(Self::Junk),
            "Galleon" => Ok(Self::Galleon),
            "Frigate" => Ok(Self::Frigate),
            _otherwise => Ok(Self::Sloop),
        }
    }
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
    pub price: i32,
}

impl Ship {
    pub fn cost_to_repair(&self) -> i32 {
        let each_hull_cost = 25;
        (each_hull_cost * (self.hull_capacity - self.hull)).into()
    }

    pub fn cost_to_hire(&self) -> i32 {
        let each_crew_member_cost = 8;
        each_crew_member_cost * (self.crew_capacity - self.crew)
    }
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
#[store(storage = "local", storage_tab_sync)]
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
        let mut player_ship = SHIPS.get(&ShipClass::Sloop).unwrap().clone();
        player_ship.name = String::from("Luna");
        Self {
            date: NaiveDate::from_ymd(1680, 1, 1),
            player: Player {
                name: String::from("Player"),
                age: 18,
                nationality: Nationality::British,
                coins: 25_000,
                ship: player_ship,
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
    HireCrew(i32),
    TakeEnemyCargo(CargoKind),
    BuyAndReplaceShip(ShipClass),
}
