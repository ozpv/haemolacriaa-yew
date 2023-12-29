use yew::prelude::*;
use yew_router::prelude::*;

use crate::{config, Route};

#[function_component(Nav)]
pub fn nav() -> Html {
    html! {
        <nav class={classes!("bg-gray-100",)}>
            <div class={classes!("w-[80%]", "h-14", "p-4", "mx-auto",)}>
                <div class={classes!("flex", "justify-between", "items-center",)}>
                    <div><p>{"haemolacriaa"}</p></div>
                    <div>
                        <ul class={classes!("flex", "items-center", "gap-[4vw]")}>
                            {
                                config::NAV_ITEMS.into_iter().map(|item| {
                                    html! {
                                        <Link<Route> to={item.route} classes={classes!("hover:text-rose-500",)}>{item.text}</Link<Route>>
                                    }
                                }).collect::<Html>()
                            }
                        </ul>
                    </div>
                    <div><p>{"right"}</p></div>
                </div>
            </div>
        </nav>
    }
}
