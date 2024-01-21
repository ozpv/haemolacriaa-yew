use yew::prelude::*;
use yew_icons::{Icon, IconId};
use yew_router::prelude::*;

use crate::{config, Route};

pub enum NavMsg {
    UpdateNavListView,
}

pub struct Nav {
    nav_listview: bool,
}

impl Component for Nav {
    type Message = NavMsg;
    type Properties = ();
    
    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            nav_listview: true,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            NavMsg::UpdateNavListView => { self.nav_listview = !self.nav_listview; true },
            _ => { true },
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let Self { nav_listview, .. } = *self;
        let isactive = if nav_listview { "hidden" } else { "" };

        html! {
            <nav class="bg-stone-900 border-stone-200">
                <div class="max-w-screen-xl flex flex-wrap items-center justify-between mx-auto p-4">
                    <span class="flex items-center text-xl text-white">{"haemolacriaa"}</span>
                    <button onclick={_ctx.link().callback(|_| NavMsg::UpdateNavListView)} class="inline-flex items-center p-2 w-10 h-10 justify-center text-sm text-stone-500 md:hidden" aria-controls="navbar-default" aria-expanded="false">
                        if nav_listview { <Icon icon_id={IconId::BootstrapList} width={"32"} height={"32"} /> }
                        else { <Icon icon_id={IconId::BootstrapXLg} width={"32"} height={"32"} /> }
                    </button>

                    <div class={classes!("w-full", "md:block", "md:w-auto", isactive)} id="navbar-default">
                        <ul class="font-medium flex flex-col p-4 md:p-0 mt-4 border border-stone-100 bg-stone-500 md:flex-row md:space-x-8 rtl:space-x-reverse md:mt-0 md:border-0 md:bg-stone-900">
                            {
                                config::NAV_ITEMS.into_iter().map(|item| {
                                    html! {
                                        <li><Link<Route> to={item.route} classes="text-white hover:text-blue-700">{item.text}</Link<Route>></li>
                                    }
                                }).collect::<Html>()
                            }
                        </ul>
                    </div>
                </div>
            </nav>
        }
    }
}


/*#[function_component(Nav)]
pub fn nav() -> Html {
    html! {
        <nav class="bg-stone-900 border-stone-200">
            <div class="max-w-screen-xl flex flex-wrap items-center justify-between mx-auto p-4">
                <span class="flex items-center text-xl text-white">{"haemolacriaa"}</span>
                <button data-collapse-toggle="navbar-default" type="button" class="inline-flex items-center p-2 w-10 h-10 justify-center text-sm text-stone-500 md:hidden" aria-controls="navbar-default" aria-expanded="false">
                    <svg class="w-5 h-5" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 17 14">
                        <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M1 1h15M1 7h15M1 13h15"/>
                    </svg>
                </button>
                <div class="hidden w-full md:block md:w-auto" id="navbar-default">
                    <ul class="font-medium flex flex-col p-4 md:p-0 mt-4 border border-stone-100 rounded-lg bg-stone-50 md:flex-row md:space-x-8 rtl:space-x-reverse md:mt-0 md:border-0 md:bg-stone-900">
                        {
                            config::NAV_ITEMS.into_iter().map(|item| {
                                html! {
                                    <li><Link<Route> to={item.route} classes="text-rose-500 hover:text-rose-900">{item.text}</Link<Route>></li>
                                }
                            }).collect::<Html>()
                        }
                    </ul>
                </div>
            </div>
        </nav>
   }
}*/
