use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub class: String,
    pub href: String,
    pub id: String,
    #[prop_or(String::new())]
    pub tooltip_content: String,
    pub children: Html,
}

#[function_component(LinkButton)]
pub fn link_button(props: &Props) -> Html {
    html! {
        <a class={classes!(props.class.clone(), "text-center", "mt-[16px]", "p-[20px]", "rounded-lg", "w-80", "bg-blue-700", "hover:text-rose-500")} href={props.href.clone()} id={props.id.clone()}>
                { props.children.clone() }
        </a>
    }
}
