use std::collections::HashMap;
use std::fmt::Display;
use std::rc::Rc;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::prelude::*;

use enum_display_derive::Display;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, PartialEq, Eq, Deserialize, Serialize, Store)]
enum Screen {
    #[default]
    MainNavigation,
    About,
    MainMenu,
    NewCharacter,
    Dock,
    DockMarket,
}

#[derive(Default, Display, Clone, PartialEq, Eq, Deserialize, Serialize, Store)]
enum Nationality {
    #[default]
    British,
    Spanish,
    French,
}

#[derive(Default, Clone, PartialEq, Eq, Deserialize, Serialize, Store)]
struct Cargo {
    name: String,
    description: String,
    price: u32,
    unit: u32,
}

#[derive(Default, Clone, PartialEq, Eq, Deserialize, Serialize, Store)]
struct CargoItems {
    wood: Cargo,
    sugar: Cargo,
}

#[derive(Default, Clone, PartialEq, Eq, Deserialize, Serialize, Store)]
enum ShipClass {
    Cutter,
    #[default]
    Sloop,
    Brig,
    Junk,
    Galleon,
    Frigate,
}

#[derive(Default, Clone, PartialEq, Eq, Deserialize, Serialize, Store)]
struct Ship {
    name: String,
    class: ShipClass,
    cargo: CargoItems,
    crew: u32,
    nationality: Nationality,
}

#[derive(Default, Clone, PartialEq, Eq, Deserialize, Serialize, Store)]
enum PortPopulation {
    Small,
    #[default]
    Medium,
    Large,
    Huge,
}

#[derive(Default, Clone, PartialEq, Eq, Deserialize, Serialize, Store)]
struct Port {
    name: String,
    description: String,
    population: PortPopulation,
    nationality: Nationality,
    cargo: CargoItems,
}

#[derive(Default, Clone, PartialEq, Eq, Deserialize, Serialize, Store)]
struct Player {
    name: String,
    age: u8,
    coins: i64,
    ship: Ship,
}

#[derive(Default, Copy, Display, Hash, Clone, PartialEq, Eq, Deserialize, Serialize, Store)]
enum Location {
    Barbados,
    #[default]
    PortRoyal,
    Nassau,
}

#[derive(Clone, PartialEq, Eq, Deserialize, Serialize, Store)]
#[store(storage = "local")]
struct Model {
    current_screen: Screen,
    player: Player,
    locations: HashMap<Location, Port>,
    current_location: Location,
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
                (
                    Location::Barbados,
                    Port {
                        name: String::from("Barbados"),
                        ..Default::default()
                    },
                ),
                (Location::PortRoyal, Port::default()),
                (Location::Nassau, Port::default()),
            ]),
            current_screen: Screen::default(),
            current_location: Location::default(),
        }
    }
}

enum Msg {
    SwitchScreen(Screen),
    SwitchPlayerLocation(Location),
    WoodBought(Location),
    SugarBought(Location),
}

impl Reducer<Model> for Msg {
    fn apply(&self, mut model: Rc<Model>) -> Rc<Model> {
        let state = Rc::make_mut(&mut model);
        match self {
            Msg::SwitchScreen(s) => state.current_screen = s.to_owned(),
            Msg::SwitchPlayerLocation(l) => state.current_location = l.to_owned(),
            Msg::WoodBought(l) => state.player.ship.cargo.wood.unit += 1,
            Msg::SugarBought(l) => state.player.ship.cargo.sugar.unit += 1,
        };

        model
    }
}

fn root_container(view: Html) -> Html {
    html! {
        <div>
        { view }
        </div>
    }
}

fn onclick_switch_screen(dispatch: &Dispatch<Model>, screen: Screen, name: &str) -> Html {
    html! {
        <button onclick={dispatch.apply_callback(move |_| Msg::SwitchScreen(screen.to_owned()))}>{name}</button>
    }
}

fn debug_header(dispatch: &Dispatch<Model>) -> Html {
    html! {
        <div>
            <h1>{"Lunar"}</h1>
            { onclick_switch_screen(dispatch, Screen::MainMenu, "Back to main menu") }
            <hr/>
        </div>
    }
}

fn show_about(model: Rc<Model>, dispatch: &Dispatch<Model>) -> Html {
    root_container(html! {
        <>
        { debug_header(dispatch) }
        <p>{"About screen"}</p>
        { onclick_switch_screen(dispatch, Screen::MainMenu, "Back") }
        </>
    })
}

fn show_main_menu(model: Rc<Model>, dispatch: &Dispatch<Model>) -> Html {
    root_container(html! {
        <>
        <p>{"Hello world!"}</p>
        { onclick_switch_screen(dispatch, Screen::NewCharacter, "Start") }
        <br/>
        { onclick_switch_screen(dispatch, Screen::About, "About") }
        </>
    })
}

fn show_new_character(model: Rc<Model>, dispatch: &Dispatch<Model>) -> Html {
    root_container(html! {
        <>
        { debug_header(dispatch) }
        <h2>{"New character"}</h2>
        <br/>
        <label>{"Name"}</label>
        <br/>
        <input placeholder="Jefferson" required=true type="text" value={model.player.name.to_owned()}
            onkeypress={dispatch.reduce_mut_callback_with(move |model, e: KeyboardEvent| {
                if e.key() == "Enter" {
                    let input: HtmlInputElement = e.target_unchecked_into();

                    model.player.name = input.value()
                }
            })}
        />
        <br/>
        { onclick_switch_screen(dispatch, Screen::MainNavigation, "Continue") }
        { onclick_switch_screen(dispatch, Screen::MainMenu, "Back") }
        </>
    })
}

fn show_main_navigation(model: Rc<Model>, dispatch: &Dispatch<Model>) -> Html {
    root_container(html! {
        <>
        { debug_header(dispatch) }
        <h2>{"Navigation page"}</h2>

        { onclick_switch_screen(dispatch, Screen::Dock, "Dock") }
        { onclick_switch_screen(dispatch, Screen::MainMenu, "Back to main menu") }

        <hr/>

        <button onclick={dispatch.apply_callback(|_| Msg::SwitchPlayerLocation(Location::Barbados) )}>{ Location::Barbados }</button>
        <button onclick={dispatch.apply_callback(|_| Msg::SwitchPlayerLocation(Location::PortRoyal) )}>{Location::PortRoyal}</button>
        <button onclick={dispatch.apply_callback(|_| Msg::SwitchPlayerLocation(Location::Nassau) )}>{Location::Nassau}</button>

        </>
    })
}

fn show_dock(model: Rc<Model>, dispatch: &Dispatch<Model>) -> Html {
    html! {
        <>
        { debug_header(dispatch) }
        <h2>{"Dock screen"}</h2>

        { onclick_switch_screen(dispatch, Screen::DockMarket, "Market") }
        </>
    }
}

fn cargo_market(model: Rc<Model>, dispatch: &Dispatch<Model>) -> Html {
    let cl = model.current_location.clone();
    html! {
        <div>
            <ul>
                <li>
                    <p>{"Wood"}</p>
                    <button onclick={dispatch.apply_callback(move |_| Msg::WoodBought(cl))}>{"Buy"}</button>
                </li>

                <li>
                    <p>{"Sugar"}</p>
                    <button onclick={dispatch.apply_callback(move |_|  Msg::SugarBought(cl))}>{"Buy"}</button>
                </li>
            </ul>
        </div>
    }
}

fn show_dock_market(model: Rc<Model>, dispatch: &Dispatch<Model>) -> Html {
    html! {
        <>
        { debug_header(dispatch) }
        <h2>{"Market screen"}</h2>

        { cargo_market(model, dispatch) }
        </>
    }
}

#[function_component]
fn View() -> Html {
    let (model, dispatch) = use_store::<Model>();

    match model.current_screen {
        Screen::About => show_about(model, &dispatch),
        Screen::MainMenu => show_main_menu(model, &dispatch),
        Screen::NewCharacter => show_new_character(model, &dispatch),
        Screen::MainNavigation => show_main_navigation(model, &dispatch),
        Screen::Dock => show_dock(model, &dispatch),
        Screen::DockMarket => show_dock_market(model, &dispatch),
    }
}

fn main() {
    yew::Renderer::<View>::new().render();
}
