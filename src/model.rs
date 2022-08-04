use std::collections::HashMap;
use std::fmt::Display;
use yewdux::prelude::*;

use enum_display_derive::Display;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, PartialEq, Eq, Deserialize, Serialize, Store)]
pub enum Screen {
    MainNavigation,
    About,
    #[default]
    MainMenu,
    NewCharacter,
    Dock,
    DockMarket,
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
    pub cargo: CargoItems,
    pub crew: u32,
    pub nationality: Nationality,
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
    pub current_screen: Screen,
    pub player: Player,
    pub locations: HashMap<Location, Port>,
    pub current_location: Location,
}

// Initializer for our whole model at launch
impl Default for Model {
    fn default() -> Self {
        Self {
            player: Player {
                name: String::from("Jameson"),
                age: 18,
                coins: 1000,
                ship: Ship {
                    name: String::from("The Duchess"),
                    crew: 12,
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
        }
    }
}

pub enum Msg {
    SwitchScreen(Screen),
    SwitchPlayerLocation(Location),
    WoodBought(Location),
    SugarBought(Location),
    WoodSold(Location),
    SugarSold(Location),
}
