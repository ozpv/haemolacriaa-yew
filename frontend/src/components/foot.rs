use yew::prelude::*;
use yew_icons::Icon;

use crate::config;

#[function_component(Foot)]
pub fn foot() -> Html {
    html! {
        <footer class="sticky top-full bg-gray-900 md:p-5">
            <div class="bg-gray-950 shadow md:rounded-lg">
                <div class="flex items-center justify-center">
                    <nav class="flex pt-[10px] gap-[2vw] mt-3 text-sm font-medium text-white">
                        {
                            config::SOCIALMEDIAINFO_ITEMS.into_iter().map(|item| {
                                html! {
                                    <a class="p-2 rounded-sm transition-all ease-in duration-75 hover:bg-gray-800" href={item.url}><Icon icon_id={item.ico} width={"16"} height={"16"} /></a>
                                }
                            }).collect::<Html>()
                        }
                    </nav>
                </div>
                <hr class="my-[16px] border-gray-800 mx-auto w-full md:w-[70%] md:my-[20px]" />
                <span class="block pb-[20px] justify-center text-center text-xs text-gray-500 md:pb-[16px]">{"Copyleft (ɔ) 2023-2024 "}
                    <a href={"/"} class="hover:underline hover:gradient-link hover:text-transparent hover:animate-gradient">{"haemolacriaa"}</a>{". All Wrongs Reserved."}
                </span>
            </div>
        </footer>
    }
}
