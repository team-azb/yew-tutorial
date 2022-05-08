use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct ButtonProps {
    pub on_click: Callback<MouseEvent>,
}

#[function_component(Button)]
pub fn button(ButtonProps { on_click }: &ButtonProps) -> Html {
    let context = use_context::<String>();
    html! {
        <button onclick={on_click}>{ format!("click ({:?})", context) }</button>
    }
}
