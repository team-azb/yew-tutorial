use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct ButtonProps {
    pub on_click: Callback<()>,
}

#[function_component(Button)]
pub fn button(ButtonProps { on_click }: &ButtonProps) -> Html {
    let callback = {
        let on_click = on_click.clone();
        Callback::from(move |_| on_click.emit(()))
    };
    let context = use_context::<String>();
    html! {
        <button onclick={callback}>{ format!("click ({:?})", context) }</button>
    }
}
