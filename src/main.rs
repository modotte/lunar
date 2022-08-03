use std::rc::Rc;
use yew::prelude::*;
use yewdux::prelude::*;

use serde::{Deserialize, Serialize};

#[derive(Default, Clone, PartialEq, Eq, Deserialize, Serialize, Store)]
enum Screen {
    #[default]
    MainNavigation,
    MainMenu,
    NewCharacter,
}

#[derive(Default, Clone, PartialEq, Eq, Deserialize, Serialize, Store)]
#[store(storage = "local")]
struct Model {
    current_screen: Screen,
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

fn root_container(model: Rc<Model>, dispatch: &Dispatch<Model>, view: Html) -> Html {
    html! {
        <div>
        { view }
        </div>
    }
}

fn show_main_menu(model: Rc<Model>, dispatch: &Dispatch<Model>) -> Html {
    root_container(
        model,
        dispatch,
        html! {
            <>
            <p>{"Hello world!"}</p>
            <button onclick={dispatch.apply_callback(|_| Msg::SwitchScreen(Screen::NewCharacter))}>{"Start"}</button>
            </>
        },
    )
}

fn show_new_character(model: Rc<Model>, dispatch: &Dispatch<Model>) -> Html {
    root_container(
        model,
        dispatch,
        html! {
            <>
            <h2>{"New character"}</h2>
            <button onclick={dispatch.apply_callback(|_| Msg::SwitchScreen(Screen::MainNavigation))}>{"Confirm & Continue"}</button>
            <button onclick={dispatch.apply_callback(|_| Msg::SwitchScreen(Screen::MainMenu))}>{"Back"}</button>
            </>
        },
    )
}

fn show_main_navigation(model: Rc<Model>, dispatch: &Dispatch<Model>) -> Html {
    root_container(
        model,
        dispatch,
        html! {
            <>
            <h2>{"Navigation page"}</h2>
            <button onclick={dispatch.apply_callback(|_| Msg::SwitchScreen(Screen::MainMenu))}>{"Back to main menu"}</button>
            </>
        },
    )
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
