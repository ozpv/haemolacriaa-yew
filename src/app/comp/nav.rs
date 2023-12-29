use yew::prelude::*;

use crate::config;

#[function_component(Nav)]
pub fn nav() -> Html {
    html! {
        <nav>
            <ul>
                {
                    config::NAV_ITEMS.into_iter().map(|item| {
                        html! {
                            <li><a href={item.href}>{item.text}</a></li>
                        }
                    }).collect::<Html>()
                }
            </ul>
        </nav>
    }
}
