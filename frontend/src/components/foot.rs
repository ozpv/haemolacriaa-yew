use yew::prelude::*;
use yew_icons::Icon;

use crate::config;

#[function_component(Foot)]
pub fn foot() -> Html {
    html! {
        <footer class="sticky top-full bg-white shadow rounded-lg sm:m-4">
            <div class="flex items-center justify-center">
                <nav class="flex pt-[14px] mt-3 text-sm font-medium text-gray-500 dark:text-gray-400 gap-[4vw]">
                    {
                        config::SOCIALMEDIAINFO_ITEMS.into_iter().map(|item| {
                            html! {
                                <a class="hover:text-rose-500" href={item.url}><Icon icon_id={item.ico} width={"16"} height={"16"} /></a>
                            }
                        }).collect::<Html>()
                    }
                </nav>
            </div>
            <hr class="my-[16px] border-gray-200 mx-auto w-[50%] dark:border-gray-700 lg:my-[20px]" />
            <span class="block pb-[14px] justify-center text-center text-xs text-gray-500 dark:text-gray-400 sm:text-sm">{"Copyleft (ɔ) 2023-2024 "}
                <a href={"/"} class="hover:underline">{"haemolacriaa"}</a>{". All Wrongs Reserved."}
            </span>
        </footer>
    }
}
