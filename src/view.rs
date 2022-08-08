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

fn onclick_styled_btn(callback: Callback<MouseEvent>, btn_text: &str) -> Html {
    html! { <button class="button" onclick={callback}>{btn_text}</button> }
}

fn onclick_switch_screen(dispatch: &Dispatch<Model>, screen: Screen, name: &str) -> Html {
    html! {
        { onclick_styled_btn(dispatch.apply_callback(move |_| Msg::SwitchScreen(screen.to_owned())), name) }
    }
}

fn onclick_switch_location(dispatch: &Dispatch<Model>, location: PortLocation, name: &str) -> Html {
    html! {
        { onclick_styled_btn(dispatch.apply_callback(move |_| Msg::SwitchPlayerLocation(location)), name) }
    }
}

// TODO: Find a way to make it work with cfg! and debug_assertions.
fn debug_header(dispatch: &Dispatch<Model>) -> Html {
    html! {
        <div>
            <h1>{"Lunar"}</h1>
            { onclick_styled_btn(dispatch.apply_callback(move |_| Msg::ResetModel), "!!!RESET MODEL!!!")}
            { onclick_switch_screen(dispatch, Screen::MainMenu, "Back to main menu") }
            <hr/>
        </div>
    }
}

fn show_about(model: Rc<Model>, dispatch: &Dispatch<Model>) -> Html {
    html! {
        <>
        { debug_header(dispatch) }
        <a target="_blank" href="https://github.com/modotte/lunar">{"Source code on Github"}</a>
        <p>{"Licensed under the GPL-3.0-or-later license"}</p>
        { onclick_switch_screen(dispatch, Screen::MainMenu, "Back") }
        </>
    }
}

fn show_main_menu(model: Rc<Model>, dispatch: &Dispatch<Model>) -> Html {
    html! {
        <>
        { onclick_switch_screen(dispatch, Screen::NewCharacter, "Start") }
        <br/>
        { onclick_switch_screen(dispatch, Screen::About, "About") }
        </>
    }
}

fn show_new_character(model: Rc<Model>, dispatch: &Dispatch<Model>) -> Html {
    html! {
        <>
        { debug_header(dispatch) }
        <label>{"Name"}</label>
        <br/>
        <input placeholder="Player" required=true type="text" value={model.player.name.to_owned()}
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
    }
}

fn player_info(model: &Rc<Model>) -> Html {
    html! {
        <>
        <p>{"Date: "} {model.date}</p>
        <p>{"Current location: "} {model.current_port_location}</p>
        <p>{"Coins: "} {model.player.coins}</p>
        <p>{"Owned Wood: "} {model.player.ship.cargos.wood.unit}</p>
        <p>{"Owned Sugar: "} {model.player.ship.cargos.sugar.unit}</p>
        <p>{"Total cargo: "} {model.player.ship.cargos.total_unit()}</p>
        <p>{"Cargo capacity: "} {model.player.ship.cargo_capacity}</p>
        </>
    }
}

fn show_main_navigation(model: Rc<Model>, dispatch: &Dispatch<Model>) -> Html {
    html! {
        <>
        { debug_header(dispatch) }
        { player_info(&model) }
        <hr/>

        { onclick_switch_screen(dispatch, Screen::Profile, "Profile") }
        { onclick_switch_screen(dispatch, Screen::Dock, "Dock") }
        { onclick_switch_screen(dispatch, Screen::Skirmish, "Skirmish")}
        { onclick_switch_screen(dispatch, Screen::MainMenu, "Main Menu") }

        <hr/>

        { onclick_switch_location(dispatch, PortLocation::Barbados, "Barbados") }
        { onclick_switch_location(dispatch, PortLocation::PortRoyal, "Port Royal") }
        { onclick_switch_location(dispatch, PortLocation::Nassau, "Nassau") }
        </>
    }
}

fn show_profile(model: Rc<Model>, dispatch: &Dispatch<Model>) -> Html {
    html! {
        <>
        { debug_header(dispatch) }
        <h2>{"Profile"}</h2>

        { onclick_switch_screen(dispatch, Screen::MainNavigation, "Back") }
        </>
    }
}

fn show_tavern(model: Rc<Model>, dispatch: &Dispatch<Model>) -> Html {
    html! {
        <>
        { debug_header(dispatch) }
        <h2>{"Tavern screen"}</h2>

        { onclick_switch_screen(dispatch, Screen::Dock, "Back") }
        </>
    }
}

fn show_dock(model: Rc<Model>, dispatch: &Dispatch<Model>) -> Html {
    html! {
        <>
        { debug_header(dispatch) }
        <h2>{"Dock screen"}</h2>

        { onclick_switch_screen(dispatch, Screen::Tavern, "Tavern") }
        { onclick_switch_screen(dispatch, Screen::DockMarket, "Market") }
        { onclick_switch_screen(dispatch, Screen::MainNavigation, "Back") }
        </>
    }
}

fn cargo_item(
    buy_callback: Callback<MouseEvent>,
    sell_callback: Callback<MouseEvent>,
    name: &str,
) -> Html {
    html! {
        <li>
            <p>{name}</p>
            { onclick_styled_btn(buy_callback, "Buy 1") }
            { onclick_styled_btn(sell_callback, "Sell 1")}
        </li>
    }
}

fn cargo_market(model: Rc<Model>, dispatch: &Dispatch<Model>) -> Html {
    let current_location = model.current_port_location.clone();

    html! {
        <div>
            <ul>
                { cargo_item(dispatch.apply_callback(move |_| Msg::BuyWood(current_location)), dispatch.apply_callback(move |_| Msg::SellWood(current_location)), "Wood") }
                { cargo_item(dispatch.apply_callback(move |_| Msg::BuySugar(current_location)), dispatch.apply_callback(move |_| Msg::SellSugar(current_location)), "Sugar") }
            </ul>
        </div>
    }
}

fn show_dock_market(model: Rc<Model>, dispatch: &Dispatch<Model>) -> Html {
    html! {
        <>
        { debug_header(dispatch) }
        { player_info(&model) }
        <hr/>
        <h2>{"Market screen"}</h2>

        { cargo_market(model, dispatch) }

        { onclick_switch_screen(dispatch, Screen::Dock, "Back") }
        </>
    }
}

fn show_skirmish(model: Rc<Model>, dispatch: &Dispatch<Model>) -> Html {
    html! {
        <>
        { debug_header(dispatch) }
        <h2>{"Skirmish"}</h2>

        { onclick_switch_screen(dispatch, Screen::SkirmishChase, "Chase") }
        { onclick_switch_screen(dispatch, Screen::MainNavigation, "Abort") }
        </>
    }
}

fn enemy_info(model: &Rc<Model>, dispatch: &Dispatch<Model>) -> Html {
    html! {
        <>
            <p>{"Enemy hull: "} {model.enemy.as_ref().unwrap().ship.hull}</p>
            <p>{"Enemy crew: "} {model.enemy.as_ref().unwrap().ship.crew}</p>
        </>
    }
}

fn show_skirmish_chase(model: Rc<Model>, dispatch: &Dispatch<Model>) -> Html {
    html! {
        <>
        { debug_header(dispatch) }
        <h2>{"Skirmish Chase"}</h2>
        { enemy_info(&model, dispatch) }
        <hr/>

        { onclick_styled_btn(dispatch.apply_callback(move |_| Msg::SkirmishChaseClose), "Close") }
        { onclick_styled_btn(dispatch.apply_callback(move |_| Msg::SkirmishChaseDistant), "Distant") }
        { onclick_styled_btn(dispatch.apply_callback(move |_| Msg::SkirmishChaseBroadside), "Broadside") }
        </>
    }
}

fn show_skirmish_loot(model: Rc<Model>, dispatch: &Dispatch<Model>) -> Html {
    html! {
        <>
        <h2>{"Loot"}</h2>

        { onclick_switch_screen(dispatch, Screen::MainNavigation, "Continue") }
        </>
    }
}

fn show_skirmish_battle(model: Rc<Model>, dispatch: &Dispatch<Model>) -> Html {
    html! {
        <>
        { debug_header(dispatch) }
        <h2>{"Battle!"}</h2>
        { enemy_info(&model, dispatch) }
        <hr/>

        { match &model.enemy {
            Some(enemy) => {
                if enemy.ship.crew < MINIMUM_SHIP_CREW.into() {
                    html! {
                        <>
                        { onclick_switch_screen(dispatch, Screen::SkirmishLoot, "Loot enemy") }
                        { onclick_switch_screen(dispatch, Screen::MainNavigation, "Continue") }
                        </>
                    }
                }
                else {
                    html! {
                        <>
                        { onclick_styled_btn(dispatch.apply_callback(move |_| Msg::SkirmishBattleSwingSword), "Sword Attack") }
                        { onclick_styled_btn(dispatch.apply_callback(move |_| Msg::SkirmishBattleShootFalconet), "Shoot Falconet volleys") }
                        </>
                    }
                }
            },
            None => { onclick_switch_screen(dispatch, Screen::MainNavigation, "Back") },
        }}
        </>
    }
}

#[function_component]
pub fn View() -> Html {
    let (model, dispatch) = use_store::<Model>();

    root_container(match model.current_screen {
        Screen::About => show_about(model, &dispatch),
        Screen::MainMenu => show_main_menu(model, &dispatch),
        Screen::NewCharacter => show_new_character(model, &dispatch),
        Screen::MainNavigation => show_main_navigation(model, &dispatch),
        Screen::Profile => show_profile(model, &dispatch),
        Screen::Tavern => show_tavern(model, &dispatch),
        Screen::Dock => show_dock(model, &dispatch),
        Screen::DockMarket => show_dock_market(model, &dispatch),
        Screen::Skirmish => show_skirmish(model, &dispatch),
        Screen::SkirmishChase => show_skirmish_chase(model, &dispatch),
        Screen::SkirmishBattle => show_skirmish_battle(model, &dispatch),
        Screen::SkirmishLoot => show_skirmish_loot(model, &dispatch),
    })
}
