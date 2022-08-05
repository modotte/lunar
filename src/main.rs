use std::{ops::AddAssign, rc::Rc};
use view::View;
use yewdux::prelude::*;

mod model;
mod view;

use chrono::Duration;
use model::{Model, Msg};

impl Reducer<Model> for Msg {
    fn apply(&self, mut model: Rc<Model>) -> Rc<Model> {
        let state = Rc::make_mut(&mut model);

        // TODO: Send alert on insufficient fund or empty cargo unit
        match self {
            Msg::SwitchScreen(s) => state.current_screen = s.to_owned(),
            Msg::SwitchPlayerLocation(l) => {
                state.date.add_assign(Duration::days(1));
                state.current_location = l.to_owned()
            }

            // We don't need to pattern match the get_mut(l)
            // because of enum as hashmap key usage
            Msg::WoodBought(l) => {
                let mut port_wood = &mut state.locations.get_mut(l).unwrap().cargo.wood;
                if state.player.coins > port_wood.price {
                    state.player.coins -= port_wood.price;
                    port_wood.unit -= 1;
                    state.player.ship.cargo.wood.unit += 1;
                }
            }
            Msg::SugarBought(l) => {
                let mut port_sugar = &mut state.locations.get_mut(l).unwrap().cargo.sugar;
                if state.player.coins > port_sugar.price {
                    state.player.coins -= port_sugar.price;
                    port_sugar.unit -= 1;
                    state.player.ship.cargo.sugar.unit += 1;
                }
            }
            Msg::WoodSold(l) => {
                let mut port_wood = &mut state.locations.get_mut(l).unwrap().cargo.wood;
                if state.player.ship.cargo.wood.unit != 0 {
                    state.player.coins += port_wood.price;
                    port_wood.unit += 1;
                    state.player.ship.cargo.wood.unit -= 1;
                }
            }
            Msg::SugarSold(l) => {
                let mut port_sugar = &mut state.locations.get_mut(l).unwrap().cargo.sugar;
                if state.player.ship.cargo.sugar.unit != 0 {
                    state.player.coins += port_sugar.price;
                    port_sugar.unit += 1;
                    state.player.ship.cargo.sugar.unit -= 1;
                }
            }
        };

        model
    }
}

fn main() {
    yew::Renderer::<View>::new().render();
}
