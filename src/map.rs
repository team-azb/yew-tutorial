use yew::prelude::*;
use wasm_bindgen::JsValue;
use web_sys::HtmlElement;
use leaflet::{LatLng, Map, TileLayer};

#[function_component(MyMap)]
pub fn map() -> Html {

    let map_ref = use_node_ref();

    use_effect_with_deps(move |map_ref| {
        if let Some(div) = map_ref.cast::<HtmlElement>() {
            let map = Map::new_with_element(&div, &JsValue::NULL);
            map.setView(&LatLng::new(35., 135.), 11.0);
            TileLayer::new(
                "https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png",
                &JsValue::NULL,
            )
            .addTo(&map);
        }
        || ()
    }, map_ref.clone());

    html! {
        <div ref={map_ref} style="height: 400px;">
        </div>
    }
}
