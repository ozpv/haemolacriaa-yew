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
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Self { nav_listview, .. } = *self;
        let isactive = if nav_listview { "hidden" } else { "" };

        html! {
            <nav class="bg-stone-900 border-stone-200">
                <div class="max-w-screen-xl flex flex-wrap items-center justify-between mx-auto p-4">
                    <span class="flex items-center text-xl text-white">{"haemolacriaa"}</span>
                    <button onclick={ctx.link().callback(|_| NavMsg::UpdateNavListView)} class="inline-flex items-center p-2 w-10 h-10 justify-center text-sm text-stone-500 md:hidden" aria-controls="navbar-default" aria-expanded="false">
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
