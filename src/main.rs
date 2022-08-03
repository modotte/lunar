use std::rc::Rc;
use yew::prelude::*;
use yewdux::{dispatch, prelude::*};

use serde::{Deserialize, Serialize};

#[derive(Default, Clone, PartialEq, Eq, Deserialize, Serialize, Store)]
enum Screen {
    #[default]
    MainNavigation,
    MainMenu,
    NewCharacter,
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

fn show_main_menu(model: Rc<Model>, dispatch: &Dispatch<Model>) -> Html {
    root_container(html! {
        <>
        <p>{"Hello world!"}</p>
        { onclick_switch_screen(dispatch, Screen::NewCharacter, "Start") }
        </>
    })
}

fn show_new_character(model: Rc<Model>, dispatch: &Dispatch<Model>) -> Html {
    root_container(html! {
        <>
        <h2>{"New character"}</h2>
        { onclick_switch_screen(dispatch, Screen::MainNavigation, "Continue") }
        { onclick_switch_screen(dispatch, Screen::MainMenu, "Back") }
        </>
    })
}

fn show_main_navigation(model: Rc<Model>, dispatch: &Dispatch<Model>) -> Html {
    root_container(html! {
        <>
        <h2>{"Navigation page"}</h2>
        { onclick_switch_screen(dispatch, Screen::MainMenu, "Back to main menu") }

        </>
    })
}

#[function_component]
fn View() -> Html {
    let (model, dispatch) = use_store::<Model>();

    match model.current_screen {
        Screen::MainMenu => show_main_menu(model, &dispatch),
        Screen::NewCharacter => show_new_character(model, &dispatch),
        Screen::MainNavigation => show_main_navigation(model, &dispatch),
    }
}

fn main() {
    yew::Renderer::<View>::new().render();
}
