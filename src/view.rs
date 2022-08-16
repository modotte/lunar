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

fn link_switch_screen(dispatch: &Dispatch<Model>, screen: Screen, name: &str) -> Html {
    html! { <a href="#" onclick={dispatch.apply_callback(move |_| Msg::SwitchScreen(screen.to_owned()))}>{name}</a>}
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
            <a target="_blank" href="https://github.com/modotte/lunar">{"Source code on Github"}</a>
            <p>{"Licensed under the GPL-3.0-or-later license"}</p>
            { onclick_switch_screen(dispatch, Screen::MainMenu, "Back") }
        </>
    }
}

fn show_main_menu(model: Rc<Model>, dispatch: &Dispatch<Model>) -> Html {
    html! {
        <div class="tile is-ancestor">
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
        <>
        { debug_header(dispatch) }
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
        </>
    }
}

fn styled_progress(id: &str, label: &str, max: i32, value: i32) -> Html {
    html! {
        <>
            <label for={id.to_string()}>{label.to_string()} {": "} {value.to_string()} {"/"} {max.to_string()}</label>
            <progress class="progress is-small" id={id.to_string()} max={max.to_string()} value={value.to_string()}></progress>
        </>
    }
}
fn player_info(model: &Rc<Model>) -> Html {
    html! {
        <>
        <h2>{"Player"}</h2>
        <p>{"Date: "} {model.date}</p>
        <p>{"Current location: "} {model.current_port_location}</p>
        <p>{"Coins: "} {model.player.coins}</p>
        <p>{"Owned Food: "} {model.player.ship.cargos.food.unit}</p>
        <p>{"Owned Wood: "} {model.player.ship.cargos.wood.unit}</p>
        <p>{"Owned Sugar: "} {model.player.ship.cargos.sugar.unit}</p>
        { styled_progress("cargos", "Cargos", model.player.ship.cargos_capacity.into(), model.player.ship.cargos.total_unit()) }
        { styled_progress("hull", "Hull", model.player.ship.hull_capacity.into(), model.player.ship.hull.into()) }
        { styled_progress("crew", "Crew", model.player.ship.crew_capacity, model.player.ship.crew) }
        <p>{"Cannons: "} {model.player.ship.cannons} {"/"} {model.player.ship.cannons_capacity}</p>
        <p>{"Ship class: "} {model.player.ship.class}</p>
        <p>{"Ship name: "} {&model.player.ship.name}</p>
        </>
    }
}

fn player_info_box(player: &Player) -> Html {
    html! {
        <div class="box is-small">
            <h4 class="title is-4">{&player.ship.name}</h4>
            <p>{"Class: "} {&player.ship.class}</p>
            { styled_progress("hull", "Hull", player.ship.hull_capacity.into(), player.ship.hull.into()) }
            { styled_progress("crew", "Crew", player.ship.crew_capacity, player.ship.crew) }
            { styled_progress("cannons", "Cannons", player.ship.cannons_capacity.into(), player.ship.cannons.into()) }
        </div>
    }
}

fn show_main_navigation(model: Rc<Model>, dispatch: &Dispatch<Model>) -> Html {
    html! {
        <div>
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
            <h2>{"Profile"}</h2>
            <hr/>

            <ul>
                <li>{"Name: "} {&model.player.name}</li>
                <li>{"Age: "} {&model.player.age}</li>
                <li>{"Nationality: "} {&model.player.nationality}</li>
            </ul>

            { onclick_switch_screen(dispatch, Screen::MainNavigation, "Back") }
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
            { player_info(&model) }

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
            <h2>{"Dock screen"}</h2>
            { onclick_switch_screen(dispatch, Screen::DockTavern, "Tavern") }
            { onclick_switch_screen(dispatch, Screen::DockMarket, "Market") }
            { onclick_switch_screen(dispatch, Screen::DockShipyard, "Shipyard") }
            { onclick_switch_screen(dispatch, Screen::MainNavigation, "Back") }
        </div>
    }
}

fn cargo_item(
    buy_callback: Callback<MouseEvent>,
    sell_callback: Callback<MouseEvent>,
    cargo: &Cargo,
    name: &str,
) -> Html {
    html! {
        <li>
            <p>{name}</p>
            <p>{"Price: "} {cargo.price}</p>
            <p>{"Available unit: "} {cargo.unit}</p>
            { onclick_styled_btn(buy_callback, "Buy 1") }
            { onclick_styled_btn(sell_callback, "Sell 1")}
        </li>
    }
}

fn cargo_market(model: Rc<Model>, dispatch: &Dispatch<Model>) -> Html {
    let inner = model;
    let current_location = inner.current_port_location;
    let food = inner.ports.get(&current_location).unwrap().cargos.food;
    let wood = inner.ports.get(&current_location).unwrap().cargos.wood;
    let sugar = inner.ports.get(&current_location).unwrap().cargos.sugar;

    html! {
        <div>
            <ul>
                { cargo_item(dispatch.apply_callback(move |_| Msg::BuyCargo(current_location, food)), dispatch.apply_callback(move |_| Msg::SellCargo(current_location, food)), &food, "Food") }
                { cargo_item(dispatch.apply_callback(move |_| Msg::BuyCargo(current_location, wood)), dispatch.apply_callback(move |_| Msg::SellCargo(current_location, wood)), &wood, "Wood") }
                { cargo_item(dispatch.apply_callback(move |_| Msg::BuyCargo(current_location, sugar)), dispatch.apply_callback(move |_| Msg::SellCargo(current_location, sugar)), &sugar, "Sugar") }
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

fn show_dock_shipyard(model: Rc<Model>, dispatch: &Dispatch<Model>) -> Html {
    html! {
        <>
        { debug_header(dispatch) }
        <hr/>


        { player_info(&model) }

        <p>{"Cost to repair: "} { &model.player.ship.cost_to_repair() }</p>
        { onclick_styled_btn(dispatch.apply_callback(move |_| Msg::BuyAndReplaceShip(ShipClass::Cutter)), "Trade your ship for a Cutter") }
        { onclick_styled_btn(dispatch.apply_callback(move |_| Msg::BuyAndReplaceShip(ShipClass::Sloop)), "Trade your ship for a Sloop") }
        { onclick_styled_btn(dispatch.apply_callback(move |_| Msg::BuyAndReplaceShip(ShipClass::Brig)), "Trade your ship for a Brig") }
        { onclick_styled_btn(dispatch.apply_callback(move |_| Msg::BuyAndReplaceShip(ShipClass::Junk)), "Trade your ship for a Junk") }
        { onclick_styled_btn(dispatch.apply_callback(move |_| Msg::BuyAndReplaceShip(ShipClass::Galleon)), "Trade your ship for a Galleon") }
        { onclick_styled_btn(dispatch.apply_callback(move |_| Msg::BuyAndReplaceShip(ShipClass::Frigate)), "Trade your ship for a Frigate") }

        { onclick_styled_btn(dispatch.apply_callback(move |_| Msg::RepairShip(model.player.coins)), "Repair all") }
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
    let enemy = model.enemy.as_ref().unwrap();
    html! {
        <>
            <h2>{"Enemy"}</h2>
            <p>{"Enemy ship name: "} {&enemy.ship.name}</p>
            <p>{"Enemy ship class: "} {&enemy.ship.class}</p>
            <p>{"Enemy ship hull: "} {&enemy.ship.hull}</p>
            <p>{"Enemy cannons: "} {&enemy.ship.cannons}</p>
            <p>{"Enemy ship crew: "} {&enemy.ship.crew}</p>
            <p>{"Enemy distance: "} {&enemy.distance}</p>
            <p>{"Enemy nationality: "} {&enemy.nationality}</p>
            <p>{"Enemy food: "} {&enemy.ship.cargos.food.unit}</p>
            <p>{"Enemy wood: "} {&enemy.ship.cargos.wood.unit}</p>
            <p>{"Enemy sugar: "} {&enemy.ship.cargos.sugar.unit}</p>
        </>
    }
}

fn show_skirmish_chase(model: Rc<Model>, dispatch: &Dispatch<Model>) -> Html {
    html! {
        <div class="tile is-ancestor">
            <div class="tile is-vertical is-8">
                <div class="tile">
                    <div class="tile is-parent is-vertical">
                        { player_info_box(&model.player) }
                    </div>

                    <div class="tile is-parent is-vertical">
                        { enemy_info(&model, dispatch) }
                    </div>

                    <div class="tile is-parent is-vertical">
                    { enemy_info(&model, dispatch) }
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
        <>
        <h2>{"Loot"}</h2>

        { player_info(&model) }
        { enemy_info(&model, dispatch) }

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
        </>
    }
}

fn show_skirmish_battle(model: Rc<Model>, dispatch: &Dispatch<Model>) -> Html {
    html! {
        <>
        { debug_header(dispatch) }
        <h2>{"Battle!"}</h2>
        { player_info(&model) }
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

fn show_game_lost(model: Rc<Model>, dispatch: &Dispatch<Model>, reason: &GameLostReason) -> Html {
    html! {
        <>
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
