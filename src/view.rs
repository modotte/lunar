use std::rc::Rc;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::model::*;

fn root_container(view: Html) -> Html {
    html! {
        <div class="container">
        { view }
        </div>
    }
}

fn onclick_switch_screen(dispatch: &Dispatch<Model>, screen: Screen, name: &str) -> Html {
    html! {
        <button class={"button"} onclick={dispatch.apply_callback(move |_| Msg::SwitchScreen(screen.to_owned()))}>{name}</button>
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

        <button class="button" onclick={dispatch.apply_callback(|_| Msg::SwitchPlayerLocation(Location::Barbados) )}>{ Location::Barbados }</button>
        <button class="button" onclick={dispatch.apply_callback(|_| Msg::SwitchPlayerLocation(Location::PortRoyal) )}>{Location::PortRoyal}</button>
        <button class="button" onclick={dispatch.apply_callback(|_| Msg::SwitchPlayerLocation(Location::Nassau) )}>{Location::Nassau}</button>

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
    let current_location = model.current_location.clone();
    html! {
        <div>
            <ul>
                <li>
                    <p>{"Wood"}</p>
                    <button onclick={dispatch.apply_callback(move |_| Msg::WoodBought(current_location))}>{"Buy 1"}</button>
                    <button onclick={dispatch.apply_callback(move |_| Msg::WoodSold(current_location))}>{"Sell 1"}</button>
                </li>

                <li>
                    <p>{"Sugar"}</p>
                    <button onclick={dispatch.apply_callback(move |_|  Msg::SugarBought(current_location))}>{"Buy 1"}</button>
                    <button onclick={dispatch.apply_callback(move |_| Msg::SugarSold(current_location))}>{"Sell 1"}</button>
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

fn show_skirmish(model: Rc<Model>, dispatch: &Dispatch<Model>) -> Html {
    html! {
        { debug_header(dispatch) }
    }
}

#[function_component]
pub fn View() -> Html {
    let (model, dispatch) = use_store::<Model>();

    match model.current_screen {
        Screen::About => show_about(model, &dispatch),
        Screen::MainMenu => show_main_menu(model, &dispatch),
        Screen::NewCharacter => show_new_character(model, &dispatch),
        Screen::MainNavigation => show_main_navigation(model, &dispatch),
        Screen::Dock => show_dock(model, &dispatch),
        Screen::DockMarket => show_dock_market(model, &dispatch),
        Screen::Skirmish => show_skirmish(model, &dispatch),
    }
}
