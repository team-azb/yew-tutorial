mod button;
mod map;

use button::Button;
use map::MyMap;
use wasm_bindgen::prelude::wasm_bindgen;
use yew::prelude::*;

#[wasm_bindgen(module = "/src/main.js")]
extern "C" {
    #[wasm_bindgen(js_name = updateTitle)]
    fn update_title(s: String);
}

#[function_component(App)]
fn app() -> Html {
    let counter = use_state(|| 0);
    let doubled_counter = use_state(|| 0);
    let context = use_state(|| "context".to_string());
    let increment = {
        let counter = counter.clone();
        Callback::from(move |_| {
            counter.set(*counter + 1);
        })
    };
    {
        let doubled_counter = doubled_counter.clone();
        use_effect_with_deps(
            move |counter: &UseStateHandle<i32>| {
                doubled_counter.set(**counter * 2);
                update_title(counter.to_string());
                move || {
                    doubled_counter.set(0);
                    update_title("0".to_string());
                }
            },
            counter.clone(),
        )
    }
    // let layer = TileLayer::new_with_options(
    //     "https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png".to_string(),
    //     serde_json::json!({}),
    // );
    html! {
        <>
            <h1>{ (*counter).to_string() }</h1>
            <h1>{ (*doubled_counter).to_string() }</h1>
            <ContextProvider<String> context={(*context).clone()}>
                <Button on_click={increment} />
            </ContextProvider<String>>
            <MyMap/>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
