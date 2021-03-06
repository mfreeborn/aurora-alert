use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct FormProps {
    pub children: Children,
    pub onsubmit: Callback<FocusEvent>,
}

#[function_component(Form)]
pub fn form(props: &FormProps) -> Html {
    html! {
        <form onsubmit={props.onsubmit.clone()} autocomplete="off" style="" class="border border-1 rounded-2 p-4">
            { for props.children.iter() }
        </form>
    }
}
