use std::collections::HashMap;
use std::fmt::Display;
use yewdux::prelude::*;

use chrono::NaiveDate;
use enum_display_derive::Display;
use serde::{Deserialize, Serialize};

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

#[derive(Clone, PartialEq, Eq, Deserialize, Serialize, Store)]
pub struct CargoItems {
    pub wood: Cargo,
    pub sugar: Cargo,
}

impl Default for CargoItems {
    fn default() -> Self {
        CargoItems {
            wood: Cargo {
                price: 40,
                ..Default::default()
            },
            sugar: Cargo {
                price: 60,
                ..Default::default()
            },
        }
    }
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

#[derive(Clone, PartialEq, Eq, Deserialize, Serialize, Store)]
pub struct Port {
    pub name: String,
    pub description: String,
    pub population: PortPopulation,
    pub nationality: Nationality,
    pub cargo: CargoItems,
}

impl Default for Port {
    fn default() -> Self {
        Port {
            cargo: CargoItems {
                wood: Cargo {
                    unit: 35,
                    ..Default::default()
                },
                sugar: Cargo {
                    unit: 70,
                    ..Default::default()
                },
            },
            name: String::from(""),
            description: String::from(""),
            population: PortPopulation::default(),
            nationality: Nationality::default(),
        }
    }
}

#[derive(Default, Clone, PartialEq, Eq, Deserialize, Serialize, Store)]
pub struct Player {
    pub name: String,
    pub age: u8,
    pub coins: u32,
    pub ship: Ship,
}

#[derive(Default, Copy, Display, Hash, Clone, PartialEq, Eq, Deserialize, Serialize, Store)]
pub enum Location {
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
    pub locations: HashMap<Location, Port>,
    pub current_location: Location,
    pub enemy: Option<Enemy>,
}

// Initializer for our whole model at launch
impl Default for Model {
    fn default() -> Self {
        Self {
            date: NaiveDate::from_ymd(1680, 01, 01),
            player: Player {
                name: String::from("Jameson"),
                age: 18,
                coins: 1000,
                ship: Ship {
                    name: String::from("The Duchess"),
                    crew: 12,
                    cargo_capacity: 32,
                    hull: 10,
                    hull_capacity: 10,
                    cannon: 4,
                    cannon_capacity: 4,
                    ..Default::default()
                },
            },
            locations: HashMap::from([
                (Location::Barbados, Port::default()),
                (Location::PortRoyal, Port::default()),
                (Location::Nassau, Port::default()),
            ]),
            current_screen: Screen::default(),
            current_location: Location::default(),
            enemy: None,
        }
    }
}

pub enum Msg {
    SwitchScreen(Screen),
    SwitchPlayerLocation(Location),
    BuyWood(Location),
    BuySugar(Location),
    SellWood(Location),
    SellSugar(Location),
    SkirmishChaseClose,
    SkirmishChaseDistant,
}
