use std::rc::Rc;
use yew::prelude::*;
use yewdux::prelude::*;

use serde::{Deserialize, Serialize};

#[derive(Default, Clone, PartialEq, Eq, Deserialize, Serialize, Store)]
#[store(storage = "local")]
struct Model {
    count: u32,
}

enum Msg {
    Increment,
    Decrement,
}

impl Reducer<Model> for Msg {
    fn apply(&self, mut model: Rc<Model>) -> Rc<Model> {
        let state = Rc::make_mut(&mut model);
        match self {
            Msg::Increment => state.count += 1,
            Msg::Decrement => state.count -= 1,
        };

        model
    }
}

#[function_component]
fn App() -> Html {
    let (model, dispatch) = use_store::<Model>();

    html! {
        <>
        <p>{ model.count }</p>
            <button onclick={dispatch.apply_callback(|_| Msg::Increment)}>{"+1"}</button>
            <button onclick={dispatch.apply_callback(|_| Msg::Decrement)}>{"-1"}</button>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
