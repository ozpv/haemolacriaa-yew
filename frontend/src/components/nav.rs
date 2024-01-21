use yew::prelude::*;
use yew_router::prelude::*;

use crate::{config, Route};

#[function_component(Nav)]
pub fn nav() -> Html {
    html! {
        <div class="bg-gray-100" id="navbar">
            <div class="w-[80%] h-14 p-4 mx-auto">
                <div class="flex justify-between items-center">
                    <p>{"haemolacriaa"}</p>
                    <nav>                            
                        <ul class={classes!("flex", "items-center", "gap-[4vw]")}>
                            {
                                config::NAV_ITEMS.into_iter().map(|item| {
                                    html! {
                                        <li><Link<Route> to={item.route} classes={classes!("hover:text-rose-500",)}>{item.text}</Link<Route>></li>
                                    }
                                }).collect::<Html>()
                            }
                        </ul>
                    </nav>
                    <p>{"right"}</p>
                </div>
            </div>
        </div>
    }
}
