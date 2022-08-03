use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::prelude::*;

use serde::{Deserialize, Serialize};

#[derive(Default, Clone, PartialEq, Eq, Deserialize, Serialize, Store)]
enum Screen {
    #[default]
    MainNavigation,
    About,
    MainMenu,
    NewCharacter,
}

#[derive(Default, Clone, PartialEq, Eq, Deserialize, Serialize, Store)]
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
enum ShipSize {
    #[default]
    Light,
    Medium,
    Heavy,
    Flag,
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
    size: ShipSize,
    class: ShipClass,
    cargo: CargoItems,
    crew: u32,
    nationality: Nationality,
}

#[derive(Default, Clone, PartialEq, Eq, Deserialize, Serialize, Store)]
struct Player {
    name: String,
    age: u8,
    coins: i64,
}

#[derive(Default, Clone, PartialEq, Eq, Deserialize, Serialize, Store)]
#[store(storage = "local")]
struct Model {
    current_screen: Screen,
    player: Player,
}

enum Msg {
    SwitchScreen(Screen),
}

impl Reducer<Model> for Msg {
    fn apply(&self, mut model: Rc<Model>) -> Rc<Model> {
        let state = Rc::make_mut(&mut model);
        match self {
            Msg::SwitchScreen(s) => state.current_screen = s.to_owned(),
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
        { onclick_switch_screen(dispatch, Screen::MainMenu, "Back to main menu") }

        </>
    })
}

#[function_component]
fn View() -> Html {
    let (model, dispatch) = use_store::<Model>();

    match model.current_screen {
        Screen::About => show_about(model, &dispatch),
        Screen::MainMenu => show_main_menu(model, &dispatch),
        Screen::NewCharacter => show_new_character(model, &dispatch),
        Screen::MainNavigation => show_main_navigation(model, &dispatch),
    }
}

fn main() {
    yew::Renderer::<View>::new().render();
}
