use yew::prelude::*;
use yew_icons::Icon;

use crate::config;

pub struct Foot;

impl Component for Foot {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <footer class={classes!("sticky", "top-full", "bg-white", "rounded-lg", "shadow", "m-4", "dark:bg-gray-800",)}>
                <div class={classes!("w-full", "mx-auto", "max-w-screen-xl", "p-4", "md:flex", "md:items-center", "md:justify-between",)}>
                    <span class={classes!("text-sm", "text-gray-500", "sm:text-center", "dark:text-gray-400",)}>{"Copyleft (ɔ) 2023-2024 "}
                        <a href={"/"} class={classes!("hover:underline",)}>{"haemolacriaa"}</a>{". All Wrongs Reserved."}</span>
                    <nav class={classes!("flex", "flex-wrap", "items-center", "mt-3", "text-sm", "font-medium", "text-gray-500", "dark:text-gray-400", "sm:mt-0", "gap-[4vw]",)}>
                        {
                            config::SOCIALMEDIAINFO_ITEMS.into_iter().map(|item| {
                                html! {
                                    <a class={classes!("hover:text-rose-500",)} href={item.url}><Icon icon_id={item.ico} width={"16"} height={"16"} /></a>
                                }
                            }).collect::<Html>()
                        }
                    </nav>
                </div>
            </footer>
        }
    }
}
