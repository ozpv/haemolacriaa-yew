use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub class: String,
    pub href: String,
    pub id: String,
    pub children: Html,
}

#[function_component(LinkButton)]
pub fn link_button(props: &Props) -> Html {
    html! {
        <a class={props.class.clone()} href={props.href.clone()} id={props.id.clone()}>
                { props.children.clone() }
        </a>
    }
}
