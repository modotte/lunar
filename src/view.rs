use std::rc::Rc;
use std::str::FromStr;
use ternop::ternary;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlElement, HtmlInputElement};
use yew::prelude::*;
use yewdux::prelude::*;

use crate::model::*;

fn root_container(view: Html) -> Html {
    html! {
        <section class="section">
            <div class="container">
                <div class="columns is-centered">
                    <div class="column is-half">
                        { view }
                    </div>
                </div>
            </div>
        </section>
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

fn link_styled(callback: Callback<MouseEvent>, link_text: &str) -> Html {
    html! { <a href="#" onclick={callback}>{link_text}</a>}
}

fn link_switch_screen(dispatch: &Dispatch<Model>, screen: Screen, name: &str) -> Html {
    html! { link_styled(dispatch.apply_callback(move |_| Msg::SwitchScreen(screen.to_owned())), name) }
}

fn link_switch_location(dispatch: &Dispatch<Model>, location: PortLocation, name: &str) -> Html {
    html! {
        html! { link_styled(dispatch.apply_callback(move |_| Msg::SwitchPlayerLocation(location.to_owned())), name) }
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
        <div>
            <a target="_blank" href="https://github.com/modotte/lunar">{"Source code on Github"}</a>
            <p>{"Licensed under the GPL-3.0-or-later license"}</p>
            { onclick_switch_screen(dispatch, Screen::MainMenu, "Back") }
        </div>
    }
}

fn show_main_menu(model: Rc<Model>, dispatch: &Dispatch<Model>) -> Html {
    html! {
        <div class="tile is-ancestor main-menu">
            <div class="tile is-vertical is-8">
                <div class="tile is-parent is-vertical">
                    <div class="tile">
                        { onclick_switch_screen(dispatch, Screen::NewCharacter, "Start") }
                    </div>
                    <div class="tile">
                        { onclick_switch_screen(dispatch, Screen::About, "About") }
                    </div>
                </div>
            </div>
        </div>
    }
}

fn show_new_character(model: Rc<Model>, dispatch: &Dispatch<Model>) -> Html {
    let player = &model.player;
    html! {
        <div>
        { debug_header(dispatch) }

        <div>
            <h3 class="title is-3">{"Create a new character!"}</h3>
        </div>

        <hr/>

        <label>{"Name"}</label>
        <br/>
        <input class="input is-small" placeholder="Player" required=true type="text" value={player.name.to_string()}
            onchange={dispatch.reduce_mut_callback_with(move |model, e: Event| {
                let input: HtmlInputElement = e.target_unchecked_into();

                model.player.name = input.value();
            })}
        />

        <br/>
        <label>{"Age"}</label>
        <br/>
        <input class="input is-small" placeholder={MINIMUM_PLAYER_AGE.to_string()} required=true type="number" min={MINIMUM_PLAYER_AGE.to_string()} max={MAXIMUM_PLAYER_AGE.to_string()} value={player.age.to_string()}
        onchange={dispatch.reduce_mut_callback_with(move |model, e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();

            match input.value().parse::<i8>() {
                Ok(age) => if (MINIMUM_PLAYER_AGE..=MAXIMUM_PLAYER_AGE).contains(&age) {
                    model.player.age = age;
                } else {
                    // Set default age
                    model.player.age = MINIMUM_PLAYER_AGE;
                    web_sys::window().unwrap().alert_with_message(format!("Cannot parse age! Only number from {} to {} is accepted", MINIMUM_PLAYER_AGE, MAXIMUM_PLAYER_AGE).as_str()).unwrap();
                }
                Err(_) => web_sys::window().unwrap().alert_with_message(format!("Cannot parse age! Only number from {} to {} is accepted", MINIMUM_PLAYER_AGE, MAXIMUM_PLAYER_AGE).as_str()).unwrap(),
            }
        })}
        />

        <br/>
        <label>{"Nationality"}</label>
        <br/>
        <div class="select is-small">
            <select oninput={dispatch.reduce_mut_callback_with(move |model, e: InputEvent|
                model.player.nationality = Nationality::from_str(&e.target_unchecked_into::<HtmlInputElement>().value()).unwrap()
            )}>
                { NATIONALITIES
                    .iter()
                    .map(|n|
                        match n {
                            Nationality::British => html!(<option selected={true}>{n}</option>),
                            _otherwise => html!(<option>{n}</option>)
                        }
                    )
                    .collect::<Html>() }
            </select>
        </div>

        <br/>
        <label>{"Your ship name"}</label>
        <br/>
        <input class="input is-small" placeholder="Luna" required=true type="text" value={player.ship.name.to_string()}
            onchange={dispatch.reduce_mut_callback_with(move |model, e: Event| {
                let input: HtmlInputElement = e.target_unchecked_into();

                model.player.ship.name = input.value();
            })}
        />

        <br/>
        <label>{"Ship Class"}</label>
        <br/>
        <div class="select is-small">
            <select oninput={dispatch.reduce_mut_callback_with(move |model, e: InputEvent| {
                let selection = e.target_unchecked_into::<HtmlInputElement>().value();
                let mut ship_choice = SHIPS.get(&ShipClass::from_str(&selection).unwrap()).unwrap().clone();
                let old_ship_name = &model.player.ship.name;
                ship_choice.name = old_ship_name.to_string();

                model.player.ship = ship_choice;
            })}>
                {
                    SHIP_CLASSES
                    .iter()
                    .map(|n|
                        match n {
                            ShipClass::Sloop => html!(<option selected={true}>{n}</option>),
                            // TODO: Limit to cutter, sloop and only brig when debug completed
                            _otherwise => html!(<option>{n}</option>)
                        }
                    )
                    .collect::<Html>() }
            </select>
        </div>
        <br/>
        <br/>
        { onclick_switch_screen(dispatch, Screen::MainNavigation, "Continue") }
        { onclick_switch_screen(dispatch, Screen::MainMenu, "Back") }
        </div>
    }
}

fn styled_progress(id: &str, label: &str, max: i32, value: i32) -> Html {
    html! {
        <div>
            <label for={id.to_string()}>{label.to_string()} {": "} {value.to_string()} {"/"} {max.to_string()}</label>
            <progress class="progress is-small" id={id.to_string()} max={max.to_string()} value={value.to_string()}></progress>
        </div>
    }
}

fn battle_participant_infobox(ship: &Ship) -> Html {
    html! {
        <div class="box is-small">
            <h4 class="title is-4">{&ship.name}</h4>
            <p>{"Class: "} {&ship.class}</p>
            { styled_progress("hull", "Hull", ship.hull_capacity.into(), ship.hull.into()) }
            { styled_progress("crew", "Crew", ship.crew_capacity, ship.crew) }
            { styled_progress("cannons", "Cannons", ship.cannons_capacity.into(), ship.cannons.into()) }
            { styled_progress("cargos", "Total cargos", ship.cargos_capacity, ship.cargos.total_unit()) }
        </div>
    }
}

fn show_main_navigation(model: Rc<Model>, dispatch: &Dispatch<Model>) -> Html {
    let current_port = model.ports.get(&model.current_port_location).unwrap();
    html! {
        <div>
            <h2 class="title is-2">{&current_port.name}</h2>
            <h3 class="subtitle is-3">{&current_port.description}</h3>

            <br/>

            <div>
                <p>{"Date: "} {&model.date}</p>
                <p>{"Coins: "} {&model.player.coins}</p>
                <p>{"Food left: "} {&model.player.ship.cargos.food.unit}</p>
            </div>

            <nav class="panel">
                <p class="panel-tabs">
                    <a>{ link_switch_screen(dispatch, Screen::Profile, "Profile") }</a>
                    <a>{ link_switch_screen(dispatch, Screen::Dock, "Dock") }</a>
                    <a>{ link_switch_screen(dispatch, Screen::Skirmish, "Skirmish") }</a>
                </p>

                <a class="panel-block">
                    { link_switch_location(dispatch, PortLocation::Barbados, "Barbados") } {" - Wealthy port"}
                </a>
                <a class="panel-block">
                    { link_switch_location(dispatch, PortLocation::PortRoyal, "Port Royal") } {" - Prosperous port"}
                </a>
                <a class="panel-block">
                    { link_switch_location(dispatch, PortLocation::Nassau, "Nassau") } {" - Vibrant port"}
                </a>
            </nav>
        </div>
    }
}

fn show_profile(model: Rc<Model>, dispatch: &Dispatch<Model>) -> Html {
    html! {
        <div>
            <nav class="breadcrumb has-arrow-separator" aria-label="breadcrumbs">
                <ul>
                    <li>{ link_switch_screen(dispatch, Screen::MainNavigation, "Navigation") }</li>
                    <li class="is-active"><a href="#" aria-current="page">{"Profile"}</a></li>
                </ul>
            </nav>

            <div class="box">
                <h2>{"Profile"}</h2>
                <hr/>

                <ul>
                    <li>{"Name: "} {&model.player.name}</li>
                    <li>{"Age: "} {&model.player.age}</li>
                    <li>{"Nationality: "} {&model.player.nationality}</li>
                </ul>
            </div>
        </div>
    }
}

fn show_dock_tavern_hire_crew(model: Rc<Model>, dispatch: &Dispatch<Model>) -> Html {
    html! {
        <div>
            <nav class="breadcrumb has-arrow-separator" aria-label="breadcrumbs">
                <ul>
                    <li>{ link_switch_screen(dispatch, Screen::MainNavigation, "Navigation") }</li>
                    <li>{ link_switch_screen(dispatch, Screen::Dock, "Dock") }</li>
                    <li>{ link_switch_screen(dispatch, Screen::DockTavern, "Tavern") }</li>
                    <li class="is-active"><a href="#" aria-current="page">{"Hire Crew"}</a></li>
                </ul>
            </nav>
            { battle_participant_infobox(&model.player.ship) }

            <p>{"Cost to hire all: "} {&model.player.ship.cost_to_hire() }</p>
            { onclick_styled_btn(dispatch.apply_callback(move |_| Msg::HireCrew(model.player.coins)), "Hire until full") }
            { onclick_switch_screen(dispatch, Screen::DockTavern, "Back") }
        </div>
    }
}

fn show_dock_tavern(model: Rc<Model>, dispatch: &Dispatch<Model>) -> Html {
    html! {
        <div>
            <nav class="breadcrumb has-arrow-separator" aria-label="breadcrumbs">
                <ul>
                    <li>{ link_switch_screen(dispatch, Screen::MainNavigation, "Navigation") }</li>
                    <li>{ link_switch_screen(dispatch, Screen::Dock, "Dock") }</li>
                    <li class="is-active"><a href="#" aria-current="page">{"Tavern"}</a></li>
                </ul>
            </nav>
            <h2>{"Tavern screen"}</h2>
            { onclick_switch_screen(dispatch, Screen::TavernHireCrew, "Hire crew") }
            { onclick_switch_screen(dispatch, Screen::Dock, "Back") }
        </div>
    }
}

fn show_dock(model: Rc<Model>, dispatch: &Dispatch<Model>) -> Html {
    html! {
        <div>
            <nav class="breadcrumb has-arrow-separator" aria-label="breadcrumbs">
                <ul>
                    <li>{ link_switch_screen(dispatch, Screen::MainNavigation, "Navigation") }</li>
                    <li class="is-active"><a href="#" aria-current="page">{"Dock"}</a></li>
                </ul>
            </nav>

            <nav class="panel">
                <p class="panel-tabs">
                    <a>{ link_switch_screen(dispatch, Screen::DockTavern, "Tavern") }</a>
                    <a>{ link_switch_screen(dispatch, Screen::DockMarket, "Market") }</a>
                    <a>{ link_switch_screen(dispatch, Screen::DockShipyard, "Shipyard") }</a>
                </p>
            </nav>
        </div>
    }
}

fn cargo_item(
    buy_callback: Callback<MouseEvent>,
    sell_callback: Callback<MouseEvent>,
    player_cargo: &Cargo,
    port_cargo: &Cargo,
    name: &str,
) -> Html {
    html! {
        <li>
            <p>{name}</p>
            <p>{"Price: "} {port_cargo.price}</p>
            <p>{"Available unit: "} {port_cargo.unit}</p>
            <p>{"In your cargo space: "} {player_cargo.unit}</p>
            { onclick_styled_btn(buy_callback, "Buy 1") }
            { onclick_styled_btn(sell_callback, "Sell 1")}
        </li>
    }
}

fn cargo_market(model: &Rc<Model>, dispatch: &Dispatch<Model>) -> Html {
    let current_location = model.current_port_location;
    let player_cargos = &model.player.ship.cargos;
    let ports = &model.ports.clone();
    let port_cargos = ports.get(&current_location).unwrap().cargos.clone();

    html! {
        <div>
            <ul>
                { cargo_item(dispatch.apply_callback(move |_| Msg::BuyCargo(current_location, port_cargos.food)), dispatch.apply_callback(move |_| Msg::SellCargo(current_location, port_cargos.food)), &player_cargos.food, &port_cargos.food, "Food") }
                { cargo_item(dispatch.apply_callback(move |_| Msg::BuyCargo(current_location, port_cargos.wood)), dispatch.apply_callback(move |_| Msg::SellCargo(current_location, port_cargos.wood)), &player_cargos.wood, &port_cargos.wood, "Wood") }
                { cargo_item(dispatch.apply_callback(move |_| Msg::BuyCargo(current_location, port_cargos.sugar)), dispatch.apply_callback(move |_| Msg::SellCargo(current_location, port_cargos.sugar)), &player_cargos.sugar, &port_cargos.sugar, "Sugar") }
            </ul>
        </div>
    }
}

fn show_dock_market(model: Rc<Model>, dispatch: &Dispatch<Model>) -> Html {
    html! {
        <div>
            <nav class="breadcrumb has-arrow-separator" aria-label="breadcrumbs">
                <ul>
                    <li>{ link_switch_screen(dispatch, Screen::MainNavigation, "Navigation") }</li>
                    <li>{ link_switch_screen(dispatch, Screen::Dock, "Dock") }</li>
                    <li class="is-active"><a href="#" aria-current="page">{"Market"}</a></li>
                </ul>
            </nav>

            <h2>{"Market"}</h2>
            <hr/>
            <div class="box">
                <p>{"Coins: "} {&model.player.coins}</p>
                <p>{ styled_progress("cargos", "Player cargos", model.player.ship.cargos_capacity.into(), model.player.ship.cargos.total_unit()) }</p>
                <br/>
                { cargo_market(&model, dispatch) }
            </div>
        </div>
    }
}

fn show_dock_shipyard(model: Rc<Model>, dispatch: &Dispatch<Model>) -> Html {
    html! {
        <div>
            <nav class="breadcrumb has-arrow-separator" aria-label="breadcrumbs">
                <ul>
                    <li>{ link_switch_screen(dispatch, Screen::MainNavigation, "Navigation") }</li>
                    <li>{ link_switch_screen(dispatch, Screen::Dock, "Dock") }</li>
                    <li class="is-active"><a href="#" aria-current="page">{"Shipyard"}</a></li>
                </ul>
            </nav>

            { battle_participant_infobox(&model.player.ship) }

            <p>{"Cost to repair: "} { &model.player.ship.cost_to_repair() }</p>
            {SHIP_CLASSES.iter().map(|x| onclick_styled_btn(dispatch.apply_callback(move |_| Msg::BuyAndReplaceShip(*x)), "")).collect::<Html>() }

            { onclick_styled_btn(dispatch.apply_callback(move |_| Msg::RepairShip(model.player.coins)), "Repair all") }
            { onclick_switch_screen(dispatch, Screen::Dock, "Back") }
        </div>
    }
}

fn show_skirmish(model: Rc<Model>, dispatch: &Dispatch<Model>) -> Html {
    html! {
        <div>
            { debug_header(dispatch) }
            <h2>{"Skirmish"}</h2>

            { onclick_switch_screen(dispatch, Screen::SkirmishChase, "Chase") }
            { onclick_switch_screen(dispatch, Screen::MainNavigation, "Abort") }
        </div>
    }
}

fn show_skirmish_chase(model: Rc<Model>, dispatch: &Dispatch<Model>) -> Html {
    html! {
        <div class="tile is-ancestor">
            <div class="tile is-vertical is-8">
                <div class="tile">
                    <div class="tile is-parent is-vertical">
                        { battle_participant_infobox(&model.player.ship) }
                    </div>

                    <div class="tile is-parent is-vertical">
                        { battle_participant_infobox(&model.enemy.as_ref().unwrap().ship) }
                    </div>
                </div>
                <hr/>

                { onclick_styled_btn(dispatch.apply_callback(move |_| Msg::SkirmishChaseClose), "Close") }
                { onclick_styled_btn(dispatch.apply_callback(move |_| Msg::SkirmishChaseDistant), "Distant") }
                { onclick_styled_btn(dispatch.apply_callback(move |_| Msg::SkirmishChaseBroadside), "Broadside") }
            </div>
        </div>
    }
}

fn show_skirmish_loot(model: Rc<Model>, dispatch: &Dispatch<Model>) -> Html {
    let player_ship = &model.player.ship;
    let enemy_cargos = &model.enemy.as_ref().unwrap().ship.cargos;

    html! {
        <div>
            <h2>{"Loot"}</h2>

            { battle_participant_infobox(&model.player.ship) }
            { battle_participant_infobox(&model.enemy.as_ref().unwrap().ship) }

            <p>
                { if enemy_cargos.total_unit() > 0 && player_ship.cargos.total_unit() < player_ship.cargos_capacity {
                html! {
                    <>
                    { ternary!(enemy_cargos.food.unit > 0, onclick_styled_btn(dispatch.apply_callback(move |_| Msg::TakeEnemyCargo(CargoKind::Food)), "Take 1"), html!()) }
                    { ternary!(enemy_cargos.wood.unit > 0, onclick_styled_btn(dispatch.apply_callback(move |_| Msg::TakeEnemyCargo(CargoKind::Wood)), "Take 1"), html!()) }
                    { ternary!(enemy_cargos.sugar.unit > 0, onclick_styled_btn(dispatch.apply_callback(move |_| Msg::TakeEnemyCargo(CargoKind::Sugar)), "Take 1"), html!()) }
                    </>
                }
                }
                else {
                    html!{ "Enemy have no loot." }
                }
            }
            </p>

            { onclick_switch_screen(dispatch, Screen::MainNavigation, "Continue") }
        </div>
    }
}

fn show_skirmish_battle(model: Rc<Model>, dispatch: &Dispatch<Model>) -> Html {
    html! {
        <div>
            { debug_header(dispatch) }
            <h2>{"Battle!"}</h2>
            { battle_participant_infobox(&model.player.ship) }
            { battle_participant_infobox(&model.enemy.as_ref().unwrap().ship) }
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
        </div>
    }
}

fn show_game_lost(model: Rc<Model>, dispatch: &Dispatch<Model>, reason: &GameLostReason) -> Html {
    html! {
        <div>
            <p>
            {
                match reason {
                    GameLostReason::ShipSunk => html!("Your ship and your crew sunk into the deep abyss of the ocean. RIP"),
                    GameLostReason::AllCrewDied => html!("All your crew died and your fate is left to your enemy. RIP"),
                    GameLostReason::FoodMutiny => html!("You ran out of food to feed your crew! One of your crew members took the matter into their own hand and managed to cause a mutiny! Where do you wanna go now? RIP "),
                }
            }
            </p>

            <br/>
            { onclick_switch_screen(dispatch, Screen::MainMenu, "Back to main menu") }
        </div>
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
        Screen::Dock => show_dock(model, &dispatch),
        Screen::DockTavern => show_dock_tavern(model, &dispatch),
        Screen::TavernHireCrew => show_dock_tavern_hire_crew(model, &dispatch),
        Screen::DockMarket => show_dock_market(model, &dispatch),
        Screen::DockShipyard => show_dock_shipyard(model, &dispatch),
        Screen::Skirmish => show_skirmish(model, &dispatch),
        Screen::SkirmishChase => show_skirmish_chase(model, &dispatch),
        Screen::SkirmishBattle => show_skirmish_battle(model, &dispatch),
        Screen::SkirmishLoot => show_skirmish_loot(model, &dispatch),
        Screen::GameLost(reason) => show_game_lost(model, &dispatch, &reason),
    })
}
