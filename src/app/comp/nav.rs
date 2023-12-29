use yew::prelude::*;

use crate::config;

#[function_component(Nav)]
pub fn nav() -> Html {
    html! {
        <nav class={classes!("bg-black")}>
            <div class={classes!("max-w-7xl", "mx-auto", "border", "border-red-400",)}>
                <ul class="">
                    {
                        config::NAV_ITEMS.into_iter().map(|item| {
                            html! {
                                <li><a href={item.href}>{item.text}</a></li>
                            }
                        }).collect::<Html>()
                    }
                </ul>
            </div>
        </nav>
    }
}
